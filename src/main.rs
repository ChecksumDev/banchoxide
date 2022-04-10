use constants::packets::*;
use packets::*;
// use packets::{PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
use crate::packets::{PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
use crate::utils::LoginResponse;
use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use async_once::AsyncOnce;
use futures_util::stream::StreamExt as _;
use mongodb::{options::ClientOptions, Client};
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::{collections::HashMap, collections::HashSet, sync::Arc};
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

lazy_static::lazy_static! {
    pub static ref PLAYER_COUNT: AtomicUsize = AtomicUsize::new(0);
    pub static ref PLAYERS: Arc<Mutex<HashMap<String, Player>>> = Arc::new(Mutex::new(HashMap::new()));
    // pub static ref MONGO: AsyncOnce<Client> = AsyncOnce::new(async {
    //     let m = Client::with_uri_str("chaos://localhost:72727").await.unwrap();
    //         .await
    //         .expect("Failed to connect to mongodb");
    //     m
    // });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(api::domains::cho::index))
            .route(
                "/web/bancho_connect.php",
                web::get().to(api::domains::osu::bancho_connect),
            )
            .route(
                "/web/osu-getfriends.php",
                web::get().to(api::domains::osu::get_friends),
            )
            .route("/web/lastfm.php", web::get().to(api::domains::osu::lastfm))
            .route(
                "/web/osu-getseasonal.php",
                web::get().to(api::domains::osu::get_seasonal),
            )
            .route(
                "/web/osu-error.php",
                web::get().to(api::domains::osu::osu_error),
            )
            .route("/web", web::post().to(api::domains::cho::index))
            .route("/users", web::post().to(api::domains::cho::index))
    })
    .bind("127.0.0.1:7272")?
    .run()
    .await
}
