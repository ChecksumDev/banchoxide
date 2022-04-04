// pub struct OsuApiv1;

// ///async def osuapiv1_getbeatmaps(
// //     **params: Union[str, int]
// // ) -> Optional[list[dict[str, Any]]]:
// //     """Fetch data from the osu!api with a beatmap's md5."""
// //     if app.settings.DEBUG:
// //         log(f"Doing osu!api (getbeatmaps) request {params}", Ansi.LMAGENTA)

// //     if not app.settings.OSU_API_KEY:
// //         return None

// //     params["k"] = str(app.settings.OSU_API_KEY)

// //     # https://github.com/ppy/osu-api/wiki#apiget_beatmaps
// //     async with app.state.services.http.get(
// //         url="https://old.ppy.sh/api/get_beatmaps",
// //         params=params,
// //     ) as resp:
// //         if resp and resp.status == 200 and resp.content.total_bytes != 2:  # b'[]'
// //             return await resp.json()

// //     return None

// impl OsuApiv1 {
//     pub fn new() -> Self {
//         OsuApiv1 {}
//     }

//     async fn get_beatmaps(beatmap_md5: &str) -> Option<Beatmap> {
//         None
//     }
// }
