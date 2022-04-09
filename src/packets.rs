extern crate alloc;

use crate::constants::packets::{self, Packets};
use crate::structs::player::{Action, Player, Status};
use actix_web::{Error, HttpResponse};
use alloc::string::String;
use alloc::vec::Vec;
use futures::future::{BoxFuture, FutureExt};
use serde::ser::Serialize;
use std::collections::HashMap;

use std::ops::{Add, AddAssign};

pub fn pack<T: serde::ser::Serialize>(data: &T) -> Vec<u8> {
    return bincode::serialize(&data).unwrap();
}

pub fn write_raw<T: Serialize>(data: T) -> Vec<u8> {
    let mut data_bytes: Vec<u8> = Vec::new();

    if std::any::type_name::<T>() == "&alloc::string::String" {
        let packet_string = unsafe { std::mem::transmute_copy::<T, &alloc::string::String>(&data) };

        data_bytes = write_osu_string(packet_string.to_string());
    } else if std::any::type_name::<T>() == "&str" {
        let packet_str = unsafe { std::mem::transmute_copy::<T, &str>(&data) };

        data_bytes = write_osu_string(packet_str.to_string());
    } else if std::any::type_name::<T>() == "&alloc::vec::Vec<i32>" {
        let int_list = unsafe { std::mem::transmute_copy::<T, &alloc::vec::Vec<i32>>(&data) };

        data_bytes.extend(pack(&(int_list.len() as u16)));
        for data_elem in int_list {
            data_bytes.extend(write_raw(data_elem));
        }
    } else if std::any::type_name::<T>() != "core::option::Option<()>" {
        data_bytes = pack(&data);
    }

    return data_bytes;
}

pub fn write_uleb128(_value: i32) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut value = _value;

    loop {
        let byte = value & 0x7f;
        value >>= 7;
        if value != 0 {
            bytes.push((byte | 0x80) as u8);
        } else {
            bytes.push(byte as u8);
            break;
        }
    }

    return bytes;
}

pub fn write_osu_string(_value: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let value = _value.as_bytes().to_vec();

    if value.is_empty() {
        bytes.push(0);
    } else {
        bytes.push(11); // 0x0B
        bytes.extend(write_uleb128(value.len() as i32));
        bytes.extend(value);
    }

    return bytes;
}

#[derive(Clone, Debug, PartialEq)]
pub struct PacketWriter {
    packet: Packets,
    data: Vec<u8>, // we barely actually need any attributes, we just like the functions.
}

impl PacketWriter {
    pub fn new(packet: Packets) -> Self {
        return Self {
            packet,
            data: Vec::new(),
        };
    }

    pub fn write<T: Serialize>(&mut self, packet_data: T) {
        self.data.extend(write_raw(packet_data));
    }

    pub fn serialise(&mut self) -> Vec<u8> {
        let mut return_data: Vec<u8> = Vec::new();

        // first add packet id
        return_data.extend(write_raw(self.packet));

        return_data.push(0); // just osu things.

        // now calculate our data length, and follow regular packet structure.
        let data_len = pack(&(self.data.len() as u32));
        return_data.extend(data_len);
        return_data.append(&mut self.data);

        return return_data;
    }
}

macro_rules! packet_impl {
    ($name: ident) => {
        impl Add<$name> for PacketWriter {
            type Output = PacketWriter;

            fn add(mut self, data: $name) -> PacketWriter {
                self.write(data);

                return self;
            }
        }

        impl AddAssign<$name> for PacketWriter {
            fn add_assign(&mut self, data: $name) {
                self.write(data);
            }
        }
    };
}

packet_impl!(u8);
packet_impl!(i16);
packet_impl!(i32);
packet_impl!(f32);
packet_impl!(i64);
packet_impl!(String);

// these ones couldn't be handled by macro :(

impl Add<Vec<u8>> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, data: Vec<u8>) -> PacketWriter {
        self.data.extend(data);

        return self;
    }
}

impl AddAssign<Vec<u8>> for PacketWriter {
    fn add_assign(&mut self, data: Vec<u8>) {
        self.data.extend(data);
    }
}

impl Add<&Vec<i32>> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, data: &Vec<i32>) -> PacketWriter {
        self.write(data);

        return self;
    }
}

impl AddAssign<&Vec<i32>> for PacketWriter {
    fn add_assign(&mut self, data: &Vec<i32>) {
        self.write(data);
    }
}

impl Add<&str> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, data: &str) -> PacketWriter {
        self.write(data);

        return self;
    }
}

impl AddAssign<&str> for PacketWriter {
    fn add_assign(&mut self, data: &str) {
        self.write(data);
    }
}

impl Add<&String> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, data: &String) -> PacketWriter {
        self.write(data);

        return self;
    }
}

impl AddAssign<&String> for PacketWriter {
    fn add_assign(&mut self, data: &String) {
        self.write(data);
    }
}

impl Add<PacketWriter> for PacketWriter {
    type Output = PacketWriter;

    fn add(mut self, writer: PacketWriter) -> PacketWriter {
        self += writer.data;

        return self;
    }
}

impl AddAssign<PacketWriter> for PacketWriter {
    fn add_assign(&mut self, writer: PacketWriter) {
        self.data.extend(writer.data);
    }
}

pub struct Reader {
    buf: Vec<u8>,
    offset: usize,
}

impl Reader {
    pub fn new(packet: Vec<u8>) -> Self {
        Self {
            buf: packet,
            offset: 0,
        }
    }

    pub fn incr_offset(&mut self, amount: usize) {
        self.offset += amount;
    }

    /// Reads a primitive type `T` from the buffer.
    pub fn read_int<T: Readable>(&mut self) -> T {
        let value = T::from_le_bytes(&self.buf[self.offset..self.offset + T::SIZE]);
        self.incr_offset(T::SIZE);
        return value;
    }

    // Maybe this should be part of read_int. Would be easily doable.
    pub fn read_f32(&mut self) -> f32 {
        let val = f32::from_le_bytes(
            self.buf[self.offset..self.offset + 4]
                .try_into()
                .expect("Should never happen."),
        );
        self.incr_offset(4);
        val
    }

    /// Reads a 128bit unsigned LEB integer from the buffer.
    pub fn read_uleb128(&mut self) -> u32 {
        let mut shift = 0_u32;
        let mut val = 0_u32;

        loop {
            let cur_byte = self.read_int::<u8>() as u32;
            val |= (cur_byte & 0b01111111) << shift;

            if cur_byte & 0b10000000 == 0 {
                break;
            }

            shift += 7;
        }
        val
    }

    /// Reads an osu style string from the buffer.
    pub fn read_str(&mut self) -> String {
        // Check exists byte.
        if self.read_int::<u8>() != 0x0b {
            return String::new();
        }

        // read string len.
        let len = self.read_uleb128() as usize;
        let string = String::from_utf8(self.buf[self.offset..self.offset + len].into())
            .unwrap_or(String::new());
        self.incr_offset(len as usize);

        string
    }

    /// Reads a list of i32s precremented by an u16 specifying length.
    pub fn read_i32_list(&mut self) -> Vec<i32> {
        let len: u16 = self.read_int();

        if len == 0 {
            return Vec::new();
        }

        let mut l = Vec::with_capacity(len as usize);
        for _ in 0..(len as usize) {
            l.push(self.read_int());
        }

        l
    }

    pub fn read_header(&mut self) -> (i32, u32) {
        let packet_id: u16 = self.read_int();

        self.incr_offset(1); // padding byte

        let packet_len: u32 = self.read_int();

        return (packet_id as i32, packet_len);
    }

    pub fn read_raw(&mut self) -> Vec<u8> {
        let len = self.buf.len() - self.offset;
        let data = &self.buf.clone()[self.offset..self.offset + len];

        self.incr_offset(len);
        return data.to_vec();
    }

    pub fn empty(&self) -> bool {
        return self.buf.len() <= self.offset;
    }
}

// FOR GENERICS!
pub trait Readable {
    fn from_le_bytes(bytes: &[u8]) -> Self;
    const SIZE: usize;
}

// Testing macros go brrrr.
macro_rules! impl_readable {
    ($name: ident) => {
        impl Readable for $name {
            fn from_le_bytes(bytes: &[u8]) -> Self {
                Self::from_le_bytes(bytes.try_into().expect("Should never happen."))
            }

            const SIZE: usize = ($name::BITS / 8) as usize;
        }
    };
}

impl Readable for u8 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        bytes[0]
    }

    const SIZE: usize = 1;
}

impl_readable!(u16);
impl_readable!(u32);
impl_readable!(u64);
impl_readable!(i8);
impl_readable!(i16);
impl_readable!(i32);
impl_readable!(i64);

/*
impl Readable for u16 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().expect("Should never happen."))
    }
    const SIZE: usize = 2;
}
*/

pub type HandlerHashMap = HashMap<
    Packets,
    for<'lt> fn(
        player: &'lt mut Player,
        reader: &'lt mut Reader,
    ) -> BoxFuture<'lt, Result<Vec<u8>, Box<dyn std::error::Error>>>,
>;

macro_rules! register_packets {(
    $(
        #[packet($id:path, $res:expr $(,)?)]
     $( #[$attr:meta] )*
        $pub:vis
        async
        fn $fname:ident ($player:ident : & $('_)? mut Player, $reader:ident : & $('_)? mut Reader) -> Result<Vec<u8>, Box<dyn std::error::Error>>
        $body:block
    )*
) => (
    $(
     $( #[$attr] )*
        $pub
        fn $fname<'lt> (
            $player : &'lt mut Player,
            $reader : &'lt mut Reader,
        ) -> BoxFuture<'lt, Result<Vec<u8>, Box<dyn std::error::Error>>>
        {
            return FutureExt::boxed(async move {
                let _ = (&$player, &$reader);
                $body
            })
        }
    )*

    lazy_static::lazy_static! {
        pub static ref PACKET_HANDLERS: HandlerHashMap = {
            let mut map = HashMap::new();
            $( map.insert($id, $fname as _); )*
            map
        };

        pub static ref RESTRICTED_PACKET_HANDLERS: HandlerHashMap = {
            let mut map = HashMap::new();
            $(
                if $res {
                    map.insert($id, $fname as _);
                }
            )*
            map
        };
    }
)}

#[macro_export]
macro_rules! write_packet {
    ( $packet:expr, $( $val:expr ),+ ) => {
        {
            let mut w = PacketWriter::new($packet);
            $(
                w += $val;
            )+
            w.serialise()
        }
    };
}

/*
pub struct ChoRestartParams {
    time_until_restart: i32,
}
pub fn cho_restart(params: ChoRestartParams) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut writer = PacketWriter::new(Packets::ChoRestart);
    writer += params.time_until_restart;

    let b = write_packet!(Packets::ChoRestart, params.time_until_restart);
    let q = write_packet!(some_packet, val);
    write_packet!(packet, params.time_until_restart);
    Ok(writer.serialise())
}
*/

register_packets! {
    // #[packet(packets::Packets::OsuPing, false)]
    // pub async fn ping(player: &mut Player, reader: &mut Reader) -> Result<HttpResponse, Error> {
    //     //println!("[DEBUG]: {} sent PING", player.name);

    //     //let mut writer = PacketWriter::new(packets::Packets::ChoPong);

    //     //let ser = writer.serialise();

    //     //Ok(HttpResponse::Ok().body(ser))
    // }

    #[packet(packets::Packets::OsuRequestStatusUpdate, false)]
    pub async fn request_status_update(player: &mut Player, reader: &mut Reader) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        println!("[PING]: {} sent RequestStatusUpdate", player.name);
        //let mut writer = PacketWriter::new(packets::Packets::ChoUserStats);

        // return write(
        //     ServerPackets.USER_STATS,
        //     (p.id, osuTypes.i32),
        //     (p.status.action, osuTypes.u8),
        //     (p.status.info_text, osuTypes.string),
        //     (p.status.map_md5, osuTypes.string),
        //     (p.status.mods, osuTypes.i32),
        //     (p.status.mode.as_vanilla, osuTypes.u8),
        //     (p.status.map_id, osuTypes.i32),
        //     (rscore, osuTypes.i64),
        //     (gm_stats.acc / 100.0, osuTypes.f32),
        //     (gm_stats.plays, osuTypes.i32),
        //     (gm_stats.tscore, osuTypes.i64),
        //     (gm_stats.rank, osuTypes.i32),
        //     (pp, osuTypes.i16),  # why not u16 peppy :(
        // )

        let ser = write_packet!(packets::Packets::ChoUserStats,
            player.id.unwrap(),
            1 as u8, "wysi".to_string(),
            "wysi".to_string(),
            1 as i32, 1 as u8,
            69420,
            727 as i64,
            69.420 as f32,
            727,
            727 as i64,
            727,
            727 as i16);
        
        Ok(ser)
    }
}
