use constants::packets::*;
use packets::*;
// use packets::{PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::stream::StreamExt as _;
use structs::player::Player;
use tokio::net::{TcpListener, TcpStream};

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

use crate::packets::{PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
use crate::utils::LoginResponse;
use async_once::AsyncOnce;
use mongodb::{options::ClientOptions, Client};
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::{collections::HashMap, collections::HashSet, sync::Arc};

lazy_static::lazy_static! {
    pub static ref PLAYER_COUNT: AtomicUsize = AtomicUsize::new(0);
    pub static ref PLAYERS: Arc<Mutex<HashMap<String, Player>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref MONGO: AsyncOnce<Client> = AsyncOnce::new(async {
        let m = Client::with_uri_str("chaos://localhost:72727").await.unwrap();
            .await
            .expect("Failed to connect to mongodb");
        m
    });
}

async fn handle_stream(data: Vec<u8>, mut player: Player) -> Result<HttpResponse, Error> {
    let mut _reader = Reader::new(data);

    while !_reader.empty() {
        let (id, len) = _reader.read_header();
        let packet = unsafe { std::mem::transmute::<i16, Packets>(id as i16) };

        //println!("Packet: {:?}", &packet);

        // &* lmao
        let mut handler_map = &*PACKET_HANDLERS;
        if player.restricted() {
            handler_map = &*RESTRICTED_PACKET_HANDLERS;
        }

        if packet == Packets::OsuPing {
            continue
        }

        if packet != Packets::OsuPing {
            println!("Packet {:?} res for {}", packet, player.name);
        }

        if handler_map.contains_key(&packet) {
            let callback = handler_map[&packet];
            let r = callback(&mut player, &mut _reader).await?;
            return Ok(HttpResponse::Ok().body(r))

            // if should_increment {
            //     _reader.incr_offset(len as usize);
            // }

            // if packet != Packets::OsuPing {
            //     println!("Packet {:?} handled for {}", packet, player.name);
            // }
        } else {
            _reader.incr_offset(len as usize);
        }
    }

    Ok((HttpResponse::Ok().body(b"".to_vec())).into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(index))
            .route("/web/bancho_connect.php", web::get().to(bancho_connect))
            .route("/web/osu-getfriends.php", web::get().to(get_friends))
            .route("/web/lastfm.php", web::get().to(lastfm))
            .route("/web/osu-getseasonal.php", web::get().to(get_seasonal))
            .route("/web/osu-error.php", web::get().to(osu_error))
            .route("/web", web::post().to(index))
            .route("/users", web::post().to(index))
    })
    .bind("127.0.0.1:7272")?
    .run()
    .await
}

async fn index(mut body: web::Payload, res: HttpRequest) -> Result<HttpResponse, Error> {
    //println!("{:?}", res.path());
    // for demonstration only; in a normal case use the `Bytes` extractor
    // collect payload stream into a bytes object
    
    let header = match res.headers().get("osu-token") {
        Some(h) => h.to_str().unwrap(),
        None => {
            // the client is preforming a login
            println!("No osu-token header found");

            let mut bytes = Vec::new();
            while let Some(item) = body.next().await {
                bytes.extend_from_slice(&item?);
            }

            let mut reader = Reader::new(bytes.clone());

            let (id, _) = reader.read_header();
            // let packet = unsafe { std::mem::transmute::<i16, Packets>(1028 as i16) };
            //println!("{:?}", packet);
            //println!("{:?}", &id);
            //println!("{:?}", String::from_utf8(bytes.clone()));

            let login_data = utils::parse_login_data(bytes.clone());

            let mut player = Player::new(
                login_data.username.as_str(),
                "727@blue.zenith",
                login_data.password_md5.as_str(),
                "US",
            );

            player.generate_token();
            player.set_id(2);

            PLAYERS
                .lock()
                .unwrap()
                .insert(player.memory.token.clone(), player.clone());
            PLAYER_COUNT.fetch_add(1, Ordering::SeqCst);

            println!("PLAYER_COUNT: {}", PLAYER_COUNT.load(Ordering::SeqCst)); // hey there cutie

            let login_res = LoginResponse::new(player.memory.token, bytes);
            // let mut data: Vec<u8> = Vec::new();
            // let mut packet_writer = packets::PacketWriter::new(Packets::ChoNotification);
            // packet_writer += "BLUE ZENITH 727!!!!".to_string();
            // data.append(&mut packet_writer.serialise());
            // let mut packet_writer = packets::PacketWriter::new(Packets::ChoRestart);
            // packet_writer += 0;
            //data.append(&mut packet_writer.serialise());



            //return Ok(HttpResponse::Ok().append_header(("cho-token", login_res.cho_token)).body(data));

            return Ok(HttpResponse::Ok()
                .append_header(("cho-token", login_res.cho_token))
                //.body("\x05\x00\x00\x04\x00\x00\x00\x7f\x7f\x7f\x7f"));
                .body(base64::decode_config("SwAABAAAABMAAAAFAAAEAAAAIQEAAEcAAAQAAAAFAAAAGAAANgAAAAs0V2VsY29tZSBiYWNrIHRvIGNteXVpLnh5eiEKUnVubmluZyBiYW5jaG8ucHkgdjQuMy4yLkEAAB0AAAALBCNvc3ULE0dlbmVyYWwgZGlzY3Vzc2lvbi4AAEEAAD4AAAALCSNhbm5vdW5jZQsvRXhlbXBsYXJ5IHBlcmZvcm1hbmNlIGFuZCBwdWJsaWMgYW5ub3VuY2VtZW50cy4AAFkAAAAAAABMAABGAAAAC0RodHRwczovL2FrYXRzdWtpLnB3L3N0YXRpYy9sb2dvcy9sb2dvX2luZ2FtZS5wbmd8aHR0cHM6Ly9ha2F0c3VraS5wd0gAAAYAAAABAAEAAABcAAAEAAAAAAAAAFMAAB0AAAAhAQAACwhjaGVja3N1bRPhAfv6xcLEMf5BAAAAAAsAAC4AAAAhAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUwAAGQAAAAEAAAALBEFpa2ET9R8AQJpEAAiHRQAAAAALAABDAAAAAQAAAAYLFGFzb3R0aWxlIHR1dG9yaWFscy4uAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAcAAJwAAAALBEFpa2ELhQFXZWxjb21lIHRvIGNteXVpLnh5ei4KVG8gc2VlIGEgbGlzdCBvZiBjb21tYW5kcywgdXNlICFoZWxwLgpXZSBoYXZlIGEgcHVibGljIChEaXNjb3JkKVtodHRwczovL2Rpc2NvcmQuZ2cvU2hFUWdVeF0hCkVuam95IHRoZSBzZXJ2ZXIhCwhjaGVja3N1bQEAAAA=", base64::URL_SAFE).unwrap()));
        }
    };

    let mut bytes = Vec::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    //println!("{:?}", String::from_utf8(bytes.clone()).unwrap());
    //println!("osu-token: {}", header);

    let player = PLAYERS
        .lock()
        .unwrap();
    let player = player.get(header)
    .expect("Player not found");

    Ok(handle_stream(bytes, player.to_owned()).await?)
    //Ok(HttpResponse::Ok().body("Hello world!"))
}

#[derive(serde::Serialize)]
struct bancho_connect_res {
    detail: String,
}

async fn bancho_connect() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Unauthorized().append_header(("bancho-version", "0.9.0"))
    .json(
        bancho_connect_res {
            detail: "Unauthorized".to_string(),
        },
    ))
}

async fn get_friends() -> Result<HttpResponse, Error> {
    //
    // @router.get("/web/osu-getfriends.php")
    // async def osuGetFriends(
    //     player: Player = Depends(authenticate_player_session(Query, "u", "h")),
    // ):
    //     return "\n".join(map(str, player.friends)).encode()

    Ok(HttpResponse::Ok().body("1"))
}

async fn get_seasonal() -> Result<HttpResponse, Error> {
    //
    // @router.get("/web/osu-getseasonal.php")
    // async def osuGetSeasonal(
    //     player: Player = Depends(authenticate_player_session(Query, "u", "h")),
    // ):
    //     return "\n".join(map(str, player.seasonal)).encode()

    Ok(HttpResponse::Ok().body("[\"https://mgo.li/c5be8df9eda37deb76de6d1621ec57c92bfb0c6e4bc1168415d2b19f333d3220901968dbf20376472edc18571779722216634cd657fa6a912cc900064a075afa.jpg?key=km0g-RksCfFT68oESmTVIN2zOh9qDRc4DrsYupAbzdo&nonce=qBE43AZJesUT_b07\"]"))
}

async fn lastfm() -> Result<HttpResponse, Error> {
    //
    // @router.get("/web/lastfm.php")
    // async def lastfm(
    //     player: Player = Depends(authenticate_player_session(Query, "u", "h")),
    // ):
    //     return "\n".join(map(str, player.lastfm)).encode()

    Ok(HttpResponse::Ok().body(b"-3".to_vec()))
}

async fn osu_error() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(b"".to_vec()))
}