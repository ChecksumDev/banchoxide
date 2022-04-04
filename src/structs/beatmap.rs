// use crate::constants::gamemodes::GameMode;
// use crate::map;
// use chrono::{DateTime, Utc};
// use std::collections::HashMap;
// use std::fmt::{Display, Formatter, Result};

// pub const BEATMAPS_PATH: &str = "./data/osu";
// pub const DEFAULT_LAST_UPDATE: &str = "1970-01-01";
// pub const IGNORED_BEATMAP_CHARS: [&str; 8] = ["\"", "/", "*", "<", ">", "?", "\"", "|"];

// bitflags::bitflags! {
//     /// Server side osu! beatmap ranked statuses.
//     /// Same as used in osu!'s /web/getscores.php.
//     pub struct RankedStatus: i32 {
//         const NOT_SUBMITTED = -1;
//         const PENDING = 0;
//         const UPDATE_AVAILABLE = 1;
//         const RANKED = 2;
//         const APPROVED = 3;
//         const QUALIFIED = 4;
//         const LOVED = 5;
//     }
// }

// impl Default for RankedStatus {
//     fn default() -> Self {
//         RankedStatus::NOT_SUBMITTED
//     }
// }

// impl Display for RankedStatus {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match self {
//             &RankedStatus::NOT_SUBMITTED => write!(f, "Unsubmitted"),
//             &RankedStatus::PENDING => write!(f, "Unranked"),
//             &RankedStatus::UPDATE_AVAILABLE => write!(f, "Outdated"),
//             &RankedStatus::RANKED => write!(f, "Ranked"),
//             &RankedStatus::APPROVED => write!(f, "Approved"),
//             &RankedStatus::QUALIFIED => write!(f, "Qualified"),
//             &RankedStatus::LOVED => write!(f, "Loved"),
//         }
//     }
// }

// impl RankedStatus {
//     /// Convert the value to osu!api status.
//     /// # Examples
//     /// ```
//     /// assert_eq!(RankedStatus::PENDING.osu_api(), 0);
//     /// assert_eq!(RankedStatus::RANKED.osu_api(), 1);
//     /// assert_eq!(RankedStatus::APPROVED.osu_api(), 2);
//     /// assert_eq!(RankedStatus::QUALIFIED.osu_api(), 3);
//     /// assert_eq!(RankedStatus::LOVED.osu_api(), 4);
//     /// ```
//     /// # Errors
//     /// This function will panic if the value is not mapped.
//     /// ```should_panic
//     /// RankedStatus::NotSubmitted.osu_api();
//     /// ```
//     pub fn osu_api(&self) -> i32 {
//         match self {
//             &RankedStatus::PENDING => 0,
//             &RankedStatus::RANKED => 1,
//             &RankedStatus::APPROVED => 2,
//             &RankedStatus::QUALIFIED => 3,
//             &RankedStatus::LOVED => 4,
//             _ => panic!("Unmapped RankedStatus: {:?}", self),
//         }
//     }

//     /// Convert from osu!api status.
//     /// # Examples
//     /// ```
//     /// assert_eq!(RankedStatus::from_osuapi(0), RankedStatus::PENDING);
//     /// assert_eq!(RankedStatus::from_osuapi(1), RankedStatus::RANKED);
//     /// assert_eq!(RankedStatus::from_osuapi(2), RankedStatus::APPROVED);
//     /// assert_eq!(RankedStatus::from_osuapi(3), RankedStatus::QUALIFIED);
//     /// assert_eq!(RankedStatus::from_osuapi(4), RankedStatus::LOVED);
//     /// ```
//     /// # Errors
//     /// This function will panic if the value is not mapped.
//     /// ```should_panic
//     /// RankedStatus::from_osuapi(5);
//     /// ```
//     pub fn from_osuapi(osuapi_status: i32) -> RankedStatus {
//         match osuapi_status {
//             -2 => RankedStatus::PENDING, // graveyard
//             -1 => RankedStatus::PENDING, // wip
//             0 => RankedStatus::PENDING,
//             1 => RankedStatus::RANKED,
//             2 => RankedStatus::APPROVED,
//             3 => RankedStatus::QUALIFIED,
//             4 => RankedStatus::LOVED,
//             _ => panic!("Unmapped osu!api status: {}", osuapi_status),
//         }
//     }

//     /// Convert from osu!direct status.
//     /// # Examples
//     /// ```
//     /// assert_eq!(RankedStatus::from_osudirect(0), RankedStatus::Ranked);
//     /// assert_eq!(RankedStatus::from_osudirect(2), RankedStatus::Pending);
//     /// assert_eq!(RankedStatus::from_osudirect(3), RankedStatus::Qualified);
//     /// assert_eq!(RankedStatus::from_osudirect(4), RankedStatus::Ranked);
//     /// assert_eq!(RankedStatus::from_osudirect(5), RankedStatus::Pending);
//     /// assert_eq!(RankedStatus::from_osudirect(7), RankedStatus::Ranked);
//     /// assert_eq!(RankedStatus::from_osudirect(8), RankedStatus::Loved);
//     /// ```
//     /// # Errors
//     /// This function will panic if the value is not mapped.
//     /// ```should_panic
//     /// RankedStatus::from_osudirect(9);
//     /// ```
//     pub fn from_osudirect(osudirect_status: i32) -> RankedStatus {
//         match osudirect_status {
//             0 => RankedStatus::RANKED,
//             2 => RankedStatus::PENDING,
//             3 => RankedStatus::QUALIFIED,
//             4 => RankedStatus::RANKED,
//             5 => RankedStatus::PENDING,
//             7 => RankedStatus::RANKED,
//             8 => RankedStatus::LOVED,
//             _ => panic!("Unmapped osu!direct status: {}", osudirect_status),
//         }
//     }

//     /// Convert from string value.
//     /// # Examples
//     /// ```
//     /// assert_eq!(RankedStatus::from_str("pending"), RankedStatus::Pending);
//     /// assert_eq!(RankedStatus::from_str("ranked"), RankedStatus::Ranked);
//     /// assert_eq!(RankedStatus::from_str("approved"), RankedStatus::Approved);
//     /// assert_eq!(RankedStatus::from_str("qualified"), RankedStatus::Qualified);
//     /// assert_eq!(RankedStatus::from_str("loved"), RankedStatus::Loved);
//     /// ```
//     /// # Errors
//     /// This function will panic if the value is not mapped.
//     /// ```should_panic
//     /// RankedStatus::from_str("unranked");
//     /// ```
//     async fn from_str(status_str: &str) -> RankedStatus {
//         match status_str {
//             "pending" => RankedStatus::PENDING,
//             "ranked" => RankedStatus::RANKED,
//             "approved" => RankedStatus::APPROVED,
//             "qualified" => RankedStatus::QUALIFIED,
//             "loved" => RankedStatus::LOVED,
//             _ => panic!("Unmapped status: {}", status_str),
//         }
//     }
// }

// /// A class to represent an osu! beatmap set.
// /// Like the Beatmap class, this class provides a high level api
// /// which should always be the preferred method of fetching beatmaps
// /// due to it's housekeeping. It will perform caching & invalidation,
// /// handle map updates while minimizing osu!api requests, and always
// /// use the most efficient method available to fetch the beatmap's
// /// information, while maintaining a low overhead.
// pub struct BeatmapSet {
//     /// The beatmap set's id.
//     pub id: i32,

//     /// The last time the beatmap set was checked for updates.
//     pub last_osuapi_check: DateTime<Utc>,

//     /// The beatmaps in the set.
//     pub maps: Vec<Beatmap>,
// }

// impl Default for BeatmapSet {
//     fn default() -> Self {
//         BeatmapSet {
//             id: 0,
//             last_osuapi_check: Utc::now(),
//             maps: Vec::new(),
//         }
//     }
// }

// impl Display for BeatmapSet {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let map_names = self
//             .maps
//             .iter()
//             .map(|bmap| format!("{} - {}", bmap.artist, bmap.title))
//             .collect::<Vec<String>>();
//         write!(f, "{}", map_names.join(", "))
//     }
// }

// pub struct Beatmap {
//     pub set: BeatmapSet,
//     pub md5: String,
//     pub id: i32,
//     pub set_id: i32,
//     pub artist: String,
//     pub title: String,
//     pub version: String,
//     pub creator: String,
//     pub last_update: String,
//     pub total_length: i32,
//     pub max_combo: i32,
//     pub status: RankedStatus,
//     pub frozen: bool,
//     pub plays: i32,
//     pub passes: i32,
//     pub mode: GameMode,
//     pub bpm: f32,
//     pub cs: f32,
//     pub od: f32,
//     pub ar: f32,
//     pub hp: f32,
//     pub diff: f32,
//     pub filename: String,
// }

// impl Default for Beatmap {
//     fn default() -> Self {
//         Beatmap {
//             set: BeatmapSet::default(),
//             md5: String::new(),
//             id: 0,
//             set_id: 0,
//             artist: String::new(),
//             title: String::new(),
//             version: String::new(),
//             creator: String::new(),
//             last_update: String::from(DEFAULT_LAST_UPDATE),
//             total_length: 0,
//             max_combo: 0,
//             status: RankedStatus::default(),
//             frozen: false,
//             plays: 0,
//             passes: 0,
//             mode: GameMode::default(),
//             bpm: 0.0,
//             cs: 0.0,
//             od: 0.0,
//             ar: 0.0,
//             hp: 0.0,
//             diff: 0.0,
//             filename: String::new(),
//         }
//     }
// }

// impl Display for Beatmap {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{} - {}", self.artist, self.title)
//     }
// }

// impl Beatmap {
//     pub fn new(
//         set: BeatmapSet,
//         md5: String,
//         id: i32,
//         set_id: i32,
//         artist: String,
//         title: String,
//         version: String,
//         creator: String,
//         last_update: String,
//         total_length: i32,
//         max_combo: i32,
//         status: RankedStatus,
//         frozen: bool,
//         plays: i32,
//         passes: i32,
//         mode: GameMode,
//         bpm: f32,
//         cs: f32,
//         od: f32,
//         ar: f32,
//         hp: f32,
//         diff: f32,
//         filename: String,
//     ) -> Self {
//         Beatmap {
//             set,
//             md5,
//             id,
//             set_id,
//             artist,
//             title,
//             version,
//             creator,
//             last_update,
//             total_length,
//             max_combo,
//             status,
//             frozen,
//             plays,
//             passes,
//             mode,
//             bpm,
//             cs,
//             od,
//             ar,
//             hp,
//             diff,
//             filename,
//         }
//     }

//     /// The full osu! formatted name `self`.
//     /// # Example
//     /// ```
//     /// beatmap.full_name()
//     /// ```
//     /// # Returns
//     /// `beatmap.artist - beatmap.title [beatmap.version]`
//     pub fn full_name(&self) -> String {
//         format!("{} - {} [{}]", self.artist, self.title, self.version)
//     }

//     /// The osu! beatmap url for `self`.
//     /// # Example
//     /// ```
//     /// beatmap.url()
//     /// ```
//     /// # Returns
//     /// `https://osu.{app.settings.DOMAIN}/beatmaps/{self.id}`
//     pub fn url(&self) -> String {
//         format!("https://osu.{}/beatmaps/{}", "checksum.space", self.id)
//     }

//     /// An osu! chat embed to `self`'s osu! beatmap page.
//     /// # Example
//     /// ```
//     /// beatmap.embed()
//     /// ```
//     /// # Returns
//     /// `[https://osu.{app.settings.DOMAIN}/beatmaps/{self.id} {self.full_name}]`
//     /// # Remarks
//     /// This is to embed the beatmap in server chat.
//     pub fn embed(&self) -> String {
//         format!("[{} {}]", self.url(), self.full_name())
//     }

//     /// Return whether the beatmap has a ranked leaderb.
//     /// # Example
//     /// ```
//     /// beatmap.has_leaderboard()
//     /// ```
//     /// # Returns
//     /// `true` if the beatmap has a ranked leaderboard, `false` otherwise.
//     pub fn has_leaderboard(&self) -> bool {
//         self.status == RankedStatus::RANKED
//             || self.status == RankedStatus::APPROVED
//             || self.status == RankedStatus::LOVED
//     }

//     /// Return whether the beatmap awards ranked pp for scores
//     /// # Example
//     /// ```
//     /// beatmap.awards_ranked_pp()
//     /// ```
//     /// # Returns
//     /// `true` if the beatmap awards ranked pp for scores, `false` otherwise.
//     pub fn awards_ranked_pp(&self) -> bool {
//         self.status == RankedStatus::RANKED || self.status == RankedStatus::APPROVED
//     }

//     //     @property  # perhaps worth caching some of?
//     //     def as_dict(self) -> dict[str, object]:
//     //         return {
//     //             "md5": self.md5,
//     //             "id": self.id,
//     //             "set_id": self.set_id,
//     //             "artist": self.artist,
//     //             "title": self.title,
//     //             "version": self.version,
//     //             "creator": self.creator,
//     //             "last_update": self.last_update,
//     //             "total_length": self.total_length,
//     //             "max_combo": self.max_combo,
//     //             "status": self.status,
//     //             "plays": self.plays,
//     //             "passes": self.passes,
//     //             "mode": self.mode,
//     //             "bpm": self.bpm,
//     //             "cs": self.cs,
//     //             "od": self.od,
//     //             "ar": self.ar,
//     //             "hp": self.hp,
//     //             "diff": self.diff,
//     //         }

//     /// Return the beatmap as a HashMap.
//     /// # Example
//     /// ```
//     /// beatmap.to_map()
//     /// ```
//     /// # Returns
//     /// `HashMap<String, String>`
//     /// # Remarks
//     /// This is to be used in the database.
//     pub fn to_map(&self) -> HashMap<&str, String> {
//         map! {
//             "md5" => self.md5,
//             "id" => self.id.to_string(),
//             "set_id" => self.set_id.to_string(),
//             "artist" => self.artist,
//             "title" => self.title,
//             "version" => self.version,
//             "creator" => self.creator,
//             "last_update" => self.last_update,
//             "total_length" => self.total_length.to_string(),
//             "max_combo" => self.max_combo.to_string(),
//             "status" => self.status.to_string(),
//             "plays" => self.plays.to_string(),
//             "passes" => self.passes.to_string(),
//             "mode" => self.mode.to_string(),
//             "bpm" => self.bpm.to_string(),
//             "cs" => self.cs.to_string(),
//             "od" => self.od.to_string(),
//             "ar" => self.ar.to_string(),
//             "hp" => self.hp.to_string(),
//             "diff" => self.diff.to_string()
//         }
//     }
// }

// //     # TODO: implement some locking for the map fetch methods

// //     """ High level API """
// //     # There are three levels of storage used for beatmaps,
// //     # the cache (ram), the db (disk), and the osu!api (web).
// //     # Going down this list gets exponentially slower, so
// //     # we always prioritze what's fastest when possible.
// //     # These methods will keep beatmaps reasonably up to
// //     # date and use the fastest storage available, while
// //     # populating the higher levels of the cache with new maps.

// //     @classmethod
// //     async def from_md5(cls, md5: str, set_id: int = -1) -> Optional[Beatmap]:
// //         """Fetch a map from the cache, database, or osuapi by md5."""
// //         bmap = await cls._from_md5_cache(md5)

// //         if not bmap:
// //             # map not found in cache

// //             # to be efficient, we want to cache the whole set
// //             # at once rather than caching the individual map

// //             if set_id <= 0:
// //                 # set id not provided - fetch it from the map md5
// //                 res = await app.state.services.database.fetch_one(
// //                     "SELECT set_id FROM maps WHERE md5 = :map_md5",
// //                     {"map_md5": md5},
// //                 )

// //                 if res is not None:
// //                     # set found in db
// //                     set_id = res["set_id"]
// //                 else:
// //                     # set not found in db, try osu!api
// //                     api_data = await osuapiv1_getbeatmaps(h=md5)

// //                     if not api_data:
// //                         return None

// //                     set_id = int(api_data[0]["beatmapset_id"])

// //             # fetch (and cache) beatmap set
// //             beatmap_set = await BeatmapSet.from_bsid(set_id)

// //             if beatmap_set is not None:
// //                 # the beatmap set has been cached - fetch beatmap from cache
// //                 bmap = await cls._from_md5_cache(md5, check_updates=False)

// //         return bmap

// //     @classmethod
// //     async def from_bid(cls, bid: int) -> Optional[Beatmap]:
// //         """Fetch a map from the cache, database, or osuapi by id."""
// //         bmap = await cls._from_bid_cache(bid)

// //         if not bmap:
// //             # map not found in cache

// //             # to be efficient, we want to cache the whole set
// //             # at once rather than caching the individual map

// //             res = await app.state.services.database.fetch_one(
// //                 "SELECT set_id FROM maps WHERE id = :map_id",
// //                 {"map_id": bid},
// //             )

// //             if res is not None:
// //                 # set found in db
// //                 set_id = res["set_id"]
// //             else:
// //                 # set not found in db, try osu!api
// //                 api_data = await osuapiv1_getbeatmaps(b=bid)

// //                 if not api_data:
// //                     return None

// //                 set_id = int(api_data[0]["beatmapset_id"])

// //             # fetch (and cache) beatmap set
// //             beatmap_set = await BeatmapSet.from_bsid(set_id)

// //             if beatmap_set is not None:
// //                 # the beatmap set has been cached - fetch beatmap from cache
// //                 bmap = await cls._from_bid_cache(bid, check_updates=False)

// //         return bmap

// //     """ Lower level API """
// //     # These functions are meant for internal use under
// //     # all normal circumstances and should only be used
// //     # if you're really modifying bancho.py by adding new
// //     # features, or perhaps optimizing parts of the code.

// //     def _parse_from_osuapi_resp(self, osuapi_resp: dict[str, Any]) -> None:
// //         """Change internal data with the data in osu!api format."""
// //         # NOTE: `self` is not guaranteed to have any attributes
// //         #       initialized when this is called.
// //         self.md5 = osuapi_resp["file_md5"]
// //         # self.id = int(osuapi_resp['beatmap_id'])
// //         self.set_id = int(osuapi_resp["beatmapset_id"])

// //         self.artist, self.title, self.version, self.creator = (
// //             osuapi_resp["artist"],
// //             osuapi_resp["title"],
// //             osuapi_resp["version"],
// //             osuapi_resp["creator"],
// //         )

// //         self.filename = (
// //             ("{artist} - {title} ({creator}) [{version}].osu")
// //             .format(**osuapi_resp)
// //             .translate(IGNORED_BEATMAP_CHARS)
// //         )

// //         # quite a bit faster than using dt.strptime.
// //         _last_update = osuapi_resp["last_update"]
// //         self.last_update = datetime(
// //             year=int(_last_update[0:4]),
// //             month=int(_last_update[5:7]),
// //             day=int(_last_update[8:10]),
// //             hour=int(_last_update[11:13]),
// //             minute=int(_last_update[14:16]),
// //             second=int(_last_update[17:19]),
// //         )

// //         self.total_length = int(osuapi_resp["total_length"])

// //         if osuapi_resp["max_combo"] is not None:
// //             self.max_combo = int(osuapi_resp["max_combo"])
// //         else:
// //             self.max_combo = 0

// //         # if a map is 'frozen', we keeps it's status
// //         # even after an update from the osu!api.
// //         if not getattr(self, "frozen", False):
// //             osuapi_status = int(osuapi_resp["approved"])
// //             self.status = RankedStatus.from_osuapi(osuapi_status)

// //         self.mode = GameMode(int(osuapi_resp["mode"]))

// //         if osuapi_resp["bpm"] is not None:
// //             self.bpm = float(osuapi_resp["bpm"])
// //         else:
// //             self.bpm = 0.0

// //         self.cs = float(osuapi_resp["diff_size"])
// //         self.od = float(osuapi_resp["diff_overall"])
// //         self.ar = float(osuapi_resp["diff_approach"])
// //         self.hp = float(osuapi_resp["diff_drain"])

// //         self.diff = float(osuapi_resp["difficultyrating"])

// //     @staticmethod
// //     async def _from_md5_cache(
// //         md5: str,
// //         check_updates: bool = True,
// //     ) -> Optional[Beatmap]:
// //         """Fetch a map from the cache by md5."""
// //         if md5 in app.state.cache.beatmap:
// //             bmap: Beatmap = app.state.cache.beatmap[md5]

// //             if check_updates and bmap.set._cache_expired():
// //                 await bmap.set._update_if_available()

// //             return bmap

// //         return None

// //     @staticmethod
// //     async def _from_bid_cache(
// //         bid: int,
// //         check_updates: bool = True,
// //     ) -> Optional[Beatmap]:
// //         """Fetch a map from the cache by id."""
// //         if bid in app.state.cache.beatmap:
// //             bmap: Beatmap = app.state.cache.beatmap[bid]

// //             if check_updates and bmap.set._cache_expired():
// //                 await bmap.set._update_if_available()

// //             return bmap

// //         return None

// //     async def fetch_rating(self) -> Optional[float]:
// //         """Fetch the beatmap's rating from sql."""
// //         row = await app.state.services.database.fetch_one(
// //             "SELECT AVG(rating) rating FROM ratings WHERE map_md5 = :map_md5",
// //             {"map_md5": self.md5},
// //         )

// //         if row is None:
// //             return None

// //         return row["rating"]
