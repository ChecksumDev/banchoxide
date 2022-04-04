use constants::packets::*;
use packets::*;
// use packets::{PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
use tokio::net::{TcpListener, TcpStream};
use futures_util::stream::StreamExt as _;
use actix_web::{web, error, App, Error, HttpResponse, HttpServer, HttpRequest};

pub mod api {
    pub mod domains {
        #[path = "api.rs"]
        pub mod api;

        #[path = "ava.rs"]
        pub mod ava;

        #[path = "cho.rs"]
        pub mod cho;

        #[path = "map.rs"]
        pub mod map;

        #[path = "osu.rs"]
        pub mod osu;
    }

    pub mod v1 {
        #[path = "osu.rs"]
        pub mod osu;
    }
}

mod constants {
    #[path = "clientflags.rs"]
    pub mod clientflags;

    #[path = "gamemodes.rs"]
    pub mod gamemodes;

    #[path = "mods.rs"]
    pub mod mods;

    #[path = "privileges.rs"]
    pub mod privileges;

    #[path = "regexes.rs"]
    pub mod regexes;

    #[path = "packets.rs"]
    pub mod packets;
}

mod structs {
    #[path = "achievement.rs"]
    pub mod achievement;

    #[path = "beatmap.rs"]
    pub mod beatmap;

    #[path = "channel.rs"]
    pub mod channel;

    #[path = "clan.rs"]
    pub mod clan;

    #[path = "collection.rs"]
    pub mod collection;

    #[path = "match.rs"]
    pub mod match_;

    #[path = "menu.rs"]
    pub mod menu;

    #[path = "models.rs"]
    pub mod models;

    #[path = "player.rs"]
    pub mod player;

    #[path = "score.rs"]
    pub mod score;
}

pub mod utils;

pub mod packets;

// Hosts; a.checksum.space, c.checksum.space, m.checksum.space, osu.checksum.space, api.checksum.space

// async fn handle_stream(mut stream: TcpStream) -> tokio::io::Result<()> {
//     let mut buf = [0u8; 1024];
//     let mut stream_buf = Vec::new();

//     stream.readable().await?;

//     while let Ok(n) = stream.try_read(&mut buf) {
//         stream_buf.extend_from_slice(&buf[..n]);
//     }

//     let _reader = Reader::new(stream_buf);



//     while !_reader.empty() {
//         let (id, len) = _reader.read_header();
//         let packet = unsafe {
//             std::mem::transmute::<i32, Packets>(id)
//         };

//         // &* lmao
//         let mut handler_map = &*PACKET_HANDLERS;
//         if player.restricted() {
//             handler_map = &*RESTRICTED_PACKET_HANDLERS;
//         }

//         if handler_map.contains_key(&packet) {
//             let callback = handler_map[&packet];
//             let should_increment = callback(&mut player, &mut _reader).await;

//             if should_increment {
//                 _reader.incr_offset(len as usize);
//             }

//             if packet != Packets::OSU_PING {
//                 println!("Packet {:?} handled for {}", packet, player.username);
//             }
//         } else {
//             _reader.incr_offset(len as usize);
//         }
//     }

//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(index))
            .route("/web", web::post().to(index))
            .route("/users", web::post().to(index))

    })
    .bind(("127.0.0.1", 443))?
    .run()
    .await
}

async fn index(mut body: web::Payload, mut res: HttpRequest) -> actix_web::Result<String> {
    // for demonstration only; in a normal case use the `Bytes` extractor
    // collect payload stream into a bytes object
    //are you there?
    let header = match res.headers().get("osu-token") {
        Some(h) => h.to_str().unwrap(),
        None => ""
    };

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    println!("{:?}", bytes);

    Ok(format!("Request Body Bytes:\n{:?}", bytes))
}