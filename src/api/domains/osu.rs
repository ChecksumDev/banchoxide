use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};

#[derive(serde::Serialize)]
struct bancho_connect_res {
    detail: String,
}

pub async fn bancho_connect() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Unauthorized()
        .append_header(("bancho-version", "0.9.0"))
        .json(bancho_connect_res {
            detail: "Unauthorized".to_string(),
        }))
}

pub async fn get_friends() -> Result<HttpResponse, Error> {
    //
    // @router.get("/web/osu-getfriends.php")
    // async def osuGetFriends(
    //     player: Player = Depends(authenticate_player_session(Query, "u", "h")),
    // ):
    //     return "\n".join(map(str, player.friends)).encode()

    Ok(HttpResponse::Ok().body("1"))
}

pub async fn get_seasonal() -> Result<HttpResponse, Error> {
    //
    // @router.get("/web/osu-getseasonal.php")
    // async def osuGetSeasonal(
    //     player: Player = Depends(authenticate_player_session(Query, "u", "h")),
    // ):
    //     return "\n".join(map(str, player.seasonal)).encode()

    Ok(HttpResponse::Ok().body("[\"https://mgo.li/c5be8df9eda37deb76de6d1621ec57c92bfb0c6e4bc1168415d2b19f333d3220901968dbf20376472edc18571779722216634cd657fa6a912cc900064a075afa.jpg?key=km0g-RksCfFT68oESmTVIN2zOh9qDRc4DrsYupAbzdo&nonce=qBE43AZJesUT_b07\"]"))
}

pub async fn lastfm() -> Result<HttpResponse, Error> {
    //
    // @router.get("/web/lastfm.php")
    // async def lastfm(
    //     player: Player = Depends(authenticate_player_session(Query, "u", "h")),
    // ):
    //     return "\n".join(map(str, player.lastfm)).encode()

    Ok(HttpResponse::Ok().body(b"-3".to_vec()))
}

pub async fn osu_error() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(b"".to_vec()))
}
