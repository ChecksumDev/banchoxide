// use bitflags::bitflags;
// use bson::oid::ObjectId;
// use chrono::{DateTime, Utc};
// use md5::{Digest, Md5};
// use std::collections::HashMap;
// use std::path::Path;
// use std::{collections, fmt::Display};

// use super::beatmap::Beatmap;
// use super::player::Player;
// use crate::constants::{clientflags::ClientFlags, gamemodes::GameMode, mods::Mods};

// const BEATMAPS_PATH: &str = ".data/osu";

// bitflags! {
//     pub struct Grade: u32 {
//         const N = 0;
//         const F = 1;
//         const D = 2;
//         const C = 3;
//         const B = 4;
//         const A = 5;
//         const S = 6;
//         const SH = 7;
//         const X = 8;
//         const XH = 9;
//     }
// }

// impl Grade {
//     pub fn from_str(s: &str) -> Grade {
//         match s.to_lowercase().as_str() {
//             "xh" => Grade::XH,
//             "x" => Grade::X,
//             "sh" => Grade::SH,
//             "s" => Grade::S,
//             "a" => Grade::A,
//             "b" => Grade::B,
//             "c" => Grade::C,
//             "d" => Grade::D,
//             "f" => Grade::F,
//             "n" => Grade::N,
//             _ => panic!("Invalid grade string"),
//         }
//     }
// }

// bitflags! {
//     pub struct SubmissionStatus: u32 {
//         const FAILED = 0;
//         const SUBMITTED = 1;
//         const BEST = 2;
//     }
// }

// impl Display for SubmissionStatus {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.bits {
//             0 => write!(f, "Failed"),
//             1 => write!(f, "Submitted"),
//             2 => write!(f, "Best"),
//             _ => write!(f, "Unknown"),
//         }
//     }
// }

// // class Score:
// //     """\
// //     Server side representation of an osu! score; any gamemode.
// //     Possibly confusing attributes
// //     -----------
// //     bmap: Optional[`Beatmap`]
// //         A beatmap obj representing the osu map.
// //     player: Optional[`Player`]
// //         A player obj of the player who submitted the score.
// //     grade: `Grade`
// //         The letter grade in the score.
// //     rank: `int`
// //         The leaderboard placement of the score.
// //     perfect: `bool`
// //         Whether the score is a full-combo.
// //     time_elapsed: `int`
// //         The total elapsed time of the play (in milliseconds).
// //     client_flags: `int`
// //         osu!'s old anticheat flags.
// //     prev_best: Optional[`Score`]
// //         The previous best score before this play was submitted.
// //         NOTE: just because a score has a `prev_best` attribute does
// //         mean the score is our best score on the map! the `status`
// //         value will always be accurate for any score.
// //     """

// //     __slots__ = (
// //         "id",
// //         "bmap",
// //         "player",
// //         "mode",
// //         "mods",
// //         "pp",
// //         "sr",
// //         "score",
// //         "max_combo",
// //         "acc",
// //         "n300",
// //         "n100",
// //         "n50",
// //         "nmiss",
// //         "ngeki",
// //         "nkatu",
// //         "grade",
// //         "rank",
// //         "passed",
// //         "perfect",
// //         "status",
// //         "client_time",
// //         "server_time",
// //         "time_elapsed",
// //         "client_flags",
// //         "client_checksum",
// //         "prev_best",
// //     )

// //     def __init__(self):
// //         # TODO: check whether the reamining Optional's should be
// //         self.id: Optional[int] = None
// //         self.bmap: Optional[Beatmap] = None
// //         self.player: Optional[Player] = None

// //         self.mode: GameMode
// //         self.mods: Mods

// //         self.pp: float
// //         self.sr: float
// //         self.score: int
// //         self.max_combo: int
// //         self.acc: float

// //         # TODO: perhaps abstract these differently
// //         # since they're mode dependant? feels weird..
// //         self.n300: int
// //         self.n100: int  # n150 for taiko
// //         self.n50: int
// //         self.nmiss: int
// //         self.ngeki: int
// //         self.nkatu: int

// //         self.grade: Grade

// //         self.passed: bool
// //         self.perfect: bool
// //         self.status: SubmissionStatus

// //         self.client_time: datetime
// //         self.server_time: datetime
// //         self.time_elapsed: int

// //         self.client_flags: ClientFlags
// //         self.client_checksum: str

// //         self.rank: Optional[int] = None
// //         self.prev_best: Optional[Score] = None

// //     def __repr__(self) -> str:
// //         # TODO: i really need to clean up my reprs
// //         try:
// //             return (
// //                 f"<{self.acc:.2f}% {self.max_combo}x {self.nmiss}M "
// //                 f"#{self.rank} on {self.bmap.full_name} for {self.pp:,.2f}pp>"
// //             )
// //         except:
// //             return super().__repr__()

// // Database entry: {  "_id": {    "$oid": "624a57b57ddf5a92d0b34216"  },  "map_md5": "wgfopwejropghj453ekr",  "score": 69727,  "pp": 69727.727,  "acc": 420.69,  "max_combo": 727,  "mods": 0,  "n300": 727,  "n100": 420,  "n50": 69,  "nmiss": 0,  "ngeki": 727,  "nkatu": 420,  "grade": "X",  "status": 0,  "mode": 0,  "play_time": {    "$date": "2000-01-01T06:00:00Z"  },  "time_elapsed": 727,  "client_flags": 2147483647,  "userid": {    "$oid": "624a537c7ddf5a92d0b341fe"  },  "perfect": true,  "online_checksum": "jeztec"}

// pub struct Score {
//     // Database fields
//     pub _id: Option<ObjectId>,
//     pub map_md5: String,
//     pub score: i32,
//     pub pp: f32,
//     pub acc: f32,
//     pub max_combo: i32,
//     pub mods: Mods,
//     pub n300: i32,
//     pub n100: i32,
//     pub n50: i32,
//     pub nmiss: i32,
//     pub ngeki: i32,
//     pub nkatu: i32,
//     pub grade: Grade,
//     pub status: SubmissionStatus,
//     pub mode: GameMode,
//     pub play_time: DateTime<Utc>,
//     pub time_elapsed: i32,
//     pub client_flags: ClientFlags,
//     pub userid: ObjectId,
//     pub perfect: bool,
//     pub online_checksum: String,

//     // Server-side only
//     pub bmap: Option<Beatmap>,
//     pub rank: Option<i32>,
//     pub prev_best: Option<Box<Score>>,
//     pub player: Option<Player>,
// }

// impl Display for Score {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "<{:.2}% {:.2}x {:.2}M #{} on {} for {:.2}pp>",
//             self.acc,
//             self.max_combo,
//             self.nmiss,
//             self.rank.as_ref().unwrap_or(&0),
//             self.bmap.as_ref().unwrap_or(&Beatmap::default()),
//             self.pp,
//         )
//     }
// }

// impl Score {
//     //     """Classmethods to fetch a score object from various data types."""
    
//     //     @classmethod
//     //     async def from_sql(cls, score_id: int) -> Optional[Score]:
//     //         """Create a score object from sql using it's scoreid."""
//     //         # XXX: perhaps in the future this should take a gamemode rather
//     //         # than just the sql table? just faster on the current setup :P
//     //         row = await app.state.services.database.fetch_one(
//     //             "SELECT id, map_md5, userid, pp, score, "
//     //             "max_combo, mods, acc, n300, n100, n50, "
//     //             "nmiss, ngeki, nkatu, grade, perfect, "
//     //             "status, mode, play_time, "
//     //             "time_elapsed, client_flags, online_checksum "
//     //             "FROM scores WHERE id = :score_id",
//     //             {"score_id": score_id},
//     //         )
    
//     //         if not row:
//     //             return None
    
//     //         s = cls()
    
//     //         s.id = row[0]
//     //         s.bmap = await Beatmap.from_md5(row[1])
//     //         s.player = await app.state.sessions.players.from_cache_or_sql(id=row[2])
    
//     //         s.sr = 0.0  # TODO
    
//     //         (
//     //             s.pp,
//     //             s.score,
//     //             s.max_combo,
//     //             s.mods,
//     //             s.acc,
//     //             s.n300,
//     //             s.n100,
//     //             s.n50,
//     //             s.nmiss,
//     //             s.ngeki,
//     //             s.nkatu,
//     //             s.grade,
//     //             s.perfect,
//     //             s.status,
//     //             s.mode,
//     //             s.server_time,
//     //             s.time_elapsed,
//     //             s.client_flags,
//     //             s.client_checksum,
//     //         ) = row[3:]
    
//     //         # fix some types
//     //         s.passed = s.status != 0
//     //         s.status = SubmissionStatus(s.status)
//     //         s.grade = Grade.from_str(s.grade)
//     //         s.mods = Mods(s.mods)
//     //         s.mode = GameMode(s.mode)
//     //         s.client_flags = ClientFlags(s.client_flags)
    
//     //         if s.bmap:
//     //             s.rank = await s.calculate_placement()
    
//     //         return s
    
//     // async fn from_mongo (score_id: ObjectId) -> Option<Score> {
//     //     let db = app.state.services.database.clone();
//     //     let score_id = score_id.clone();
//     //     let score = db.collection("scores").find_one(
//     //         Some(doc! {
//     //             "_id": score_id
//     //         }),
//     //         None,
//     //     )
//     //     .await
//     //     .ok()?;
//     // }
// }


// //     @classmethod
// //     def from_submission(cls, data: list[str]) -> Score:
// //         """Create a score object from an osu! submission string."""
// //         s = cls()

// //         """ parse the following format
// //         # 0  online_checksum
// //         # 1  n300
// //         # 2  n100
// //         # 3  n50
// //         # 4  ngeki
// //         # 5  nkatu
// //         # 6  nmiss
// //         # 7  score
// //         # 8  max_combo
// //         # 9  perfect
// //         # 10 grade
// //         # 11 mods
// //         # 12 passed
// //         # 13 gamemode
// //         # 14 play_time # yyMMddHHmmss
// //         # 15 osu_version + (" " * client_flags)
// //         """

// //         s.client_checksum = data[0]
// //         s.n300 = int(data[1])
// //         s.n100 = int(data[2])
// //         s.n50 = int(data[3])
// //         s.ngeki = int(data[4])
// //         s.nkatu = int(data[5])
// //         s.nmiss = int(data[6])
// //         s.score = int(data[7])
// //         s.max_combo = int(data[8])
// //         s.perfect = data[9] == "True"
// //         s.grade = Grade.from_str(data[10])
// //         s.mods = Mods(int(data[11]))
// //         s.passed = data[12] == "True"
// //         s.mode = GameMode.from_params(int(data[13]), s.mods)
// //         s.client_time = datetime.strptime(data[14], "%y%m%d%H%M%S")
// //         s.client_flags = ClientFlags(data[15].count(" ") & ~4)

// //         s.server_time = datetime.now()

// //         return s

// //     def compute_online_checksum(
// //         self,
// //         osu_version: str,
// //         osu_client_hash: str,
// //         storyboard_checksum: str,
// //     ) -> str:
// //         """Validate the online checksum of the score."""
// //         return hashlib.md5(
// //             "chickenmcnuggets{0}o15{1}{2}smustard{3}{4}uu{5}{6}{7}{8}{9}{10}{11}Q{12}{13}{15}{14:%y%m%d%H%M%S}{16}{17}".format(
// //                 self.n100 + self.n300,
// //                 self.n50,
// //                 self.ngeki,
// //                 self.nkatu,
// //                 self.nmiss,
// //                 self.bmap.md5,
// //                 self.max_combo,
// //                 self.perfect,
// //                 self.player.name,
// //                 self.score,
// //                 self.grade.name,
// //                 int(self.mods),
// //                 self.passed,
// //                 self.mode.as_vanilla,
// //                 self.client_time,
// //                 osu_version,  # 20210520
// //                 osu_client_hash,
// //                 storyboard_checksum,
// //                 # yyMMddHHmmss
// //             ).encode(),
// //         ).hexdigest()

// //     """Methods to calculate internal data for a score."""

// //     async def calculate_placement(self) -> int:
// //         if self.mode >= GameMode.RELAX_OSU:
// //             scoring_metric = "pp"
// //             score = self.pp
// //         else:
// //             scoring_metric = "score"
// //             score = self.score

// //         better_scores = await app.state.services.database.fetch_val(
// //             "SELECT COUNT(*) AS c FROM scores s "
// //             "INNER JOIN users u ON u.id = s.userid "
// //             "WHERE s.map_md5 = :map_md5 AND s.mode = :mode "
// //             "AND s.status = 2 AND u.priv & 1 "
// //             f"AND s.{scoring_metric} > :score",
// //             {
// //                 "map_md5": self.bmap.md5,
// //                 "mode": self.mode,
// //                 "score": score,
// //             },
// //             column=0,  # COUNT(*)
// //         )

// //         # TODO: idk if returns none
// //         return better_scores + 1  # if better_scores is not None else 1

// //     def calculate_performance(self, osu_file_path: Path) -> tuple[float, float]:
// //         """Calculate PP and star rating for our score."""
// //         mode_vn = self.mode.as_vanilla

// //         if mode_vn in (0, 1, 2):
// //             score_args: ScoreDifficultyParams = {
// //                 "acc": self.acc,
// //                 "combo": self.max_combo,
// //                 "nmiss": self.nmiss,
// //             }
// //         else:  # mode_vn == 3
// //             score_args: ScoreDifficultyParams = {
// //                 "score": self.score,
// //             }

// //         result = app.usecases.performance.calculate_performances(
// //             osu_file_path=str(osu_file_path),
// //             mode=mode_vn,
// //             mods=int(self.mods),
// //             scores=[score_args],
// //         )

// //         return result[0]["performance"], result[0]["star_rating"]

// //     async def calculate_status(self) -> None:
// //         """Calculate the submission status of a submitted score."""
// //         # find any other `status = 2` scores we have
// //         # on the map. If there are any, store
// //         res = await app.state.services.database.fetch_one(
// //             "SELECT id, pp FROM scores "
// //             "WHERE userid = :user_id AND map_md5 = :map_md5 "
// //             "AND mode = :mode AND status = 2",
// //             {
// //                 "user_id": self.player.id,
// //                 "map_md5": self.bmap.md5,
// //                 "mode": self.mode,
// //             },
// //         )

// //         if res:
// //             # we have a score on the map.
// //             # save it as our previous best score.
// //             self.prev_best = await Score.from_sql(res["id"])

// //             # if our new score is better, update
// //             # both of our score's submission statuses.
// //             # NOTE: this will be updated in sql later on in submission
// //             if self.pp > res["pp"]:
// //                 self.status = SubmissionStatus.BEST
// //                 self.prev_best.status = SubmissionStatus.SUBMITTED
// //             else:
// //                 self.status = SubmissionStatus.SUBMITTED
// //         else:
// //             # this is our first score on the map.
// //             self.status = SubmissionStatus.BEST

// //     def calculate_accuracy(self) -> float:
// //         """Calculate the accuracy of our score."""
// //         mode_vn = self.mode.as_vanilla

// //         if mode_vn == 0:  # osu!
// //             total = self.n300 + self.n100 + self.n50 + self.nmiss

// //             if total == 0:
// //                 return 0.0

// //             return (
// //                 100.0
// //                 * ((self.n300 * 300.0) + (self.n100 * 100.0) + (self.n50 * 50.0))
// //                 / (total * 300.0)
// //             )

// //         elif mode_vn == 1:  # osu!taiko
// //             total = self.n300 + self.n100 + self.nmiss

// //             if total == 0:
// //                 return 0.0

// //             return 100.0 * ((self.n100 * 0.5) + self.n300) / total

// //         elif mode_vn == 2:  # osu!catch
// //             total = self.n300 + self.n100 + self.n50 + self.nkatu + self.nmiss

// //             if total == 0:
// //                 return 0.0

// //             return 100.0 * (self.n300 + self.n100 + self.n50) / total

// //         elif mode_vn == 3:  # osu!mania
// //             total = (
// //                 self.n300 + self.n100 + self.n50 + self.ngeki + self.nkatu + self.nmiss
// //             )

// //             if total == 0:
// //                 return 0.0

// //             return (
// //                 100.0
// //                 * (
// //                     (self.n50 * 50.0)
// //                     + (self.n100 * 100.0)
// //                     + (self.nkatu * 200.0)
// //                     + ((self.n300 + self.ngeki) * 300.0)
// //                 )
// //                 / (total * 300.0)
// //             )
// //         else:
// //             raise Exception(f"Invalid vanilla mode {mode_vn}")

// //     """ Methods for updating a score. """

// //     async def increment_replay_views(self) -> None:
// //         # TODO: move replay views to be per-score rather than per-user
// //         assert self.player is not None

// //         await app.state.services.database.execute(
// //             f"UPDATE stats "
// //             "SET replay_views = replay_views + 1 "
// //             "WHERE id = :user_id AND mode = :mode",
// //             {"user_id": self.player.id, "mode": self.mode},
// //         )
