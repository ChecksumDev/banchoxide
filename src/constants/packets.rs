use crate::packets::PacketWriter;
use crate::write_packet;
use serde::{Serialize, Serializer};
use std::error::Error;

//? ===== Prefixes =====
//? Osu: Client -> Server
//? Cho: Server -> Client
//? ===== Prefixes =====

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i16)]
pub enum Packets {
    OsuChangeAction = 0,
    OsuSendPublicMessage = 1,
    OsuLogout = 2,
    OsuRequestStatusUpdate = 3,
    OsuPing = 4,
    ChoUserId = 5,                  //
    ChoSendMessage = 7,             //
    ChoPong = 8,                    //
    ChoHandleIrcChangeUsername = 9, //
    ChoHandleIrcQuit = 10,
    ChoUserStats = 11,       //
    ChoUserLogout = 12,      //
    ChoSpectatorJoined = 13, //
    ChoSpectatorLeft = 14,   //
    ChoSpectateFrames = 15,  //
    OsuStartSpectating = 16,
    OsuStopSpectating = 17,
    OsuSpectateFrames = 18,
    ChoVersionUpdate = 19, //
    OsuErrorReport = 20,
    OsuCantSpectate = 21,
    ChoSpectatorCantSpectate = 22, //
    ChoGetAttention = 23,          //
    ChoNotification = 24,          //
    OsuSendPrivateMessage = 25,
    ChoUpdateMatch = 26,
    ChoNewMatch = 27,
    ChoDisposeMatch = 28, //
    OsuPartLobby = 29,
    OsuJoinLobby = 30,
    OsuCreateMatch = 31,
    OsuJoinMatch = 32,
    OsuPartMatch = 33,
    ChoToggleBlockNonFriendDms = 34, //
    ChoMatchJoinSuccess = 36,
    ChoMatchJoinFail = 37, //
    OsuMatchChangeSlot = 38,
    OsuMatchReady = 39,
    OsuMatchLock = 40,
    OsuMatchChangeSettings = 41,
    ChoFellowSpectatorJoined = 42, //
    ChoFellowSpectatorLeft = 43,   //
    OsuMatchStart = 44,
    ChoAllPlayersLoaded = 45,
    ChoMatchStart = 46,
    OsuMatchScoreUpdate = 47,
    ChoMatchScoreUpdate = 48,
    OsuMatchComplete = 49,
    ChoMatchTransferHost = 50, //
    OsuMatchChangeMods = 51,
    OsuMatchLoadComplete = 52,
    ChoMatchAllPlayersLoaded = 53, //
    OsuMatchNoBeatmap = 54,
    OsuMatchNotReady = 55,
    OsuMatchFailed = 56,
    ChoMatchPlayerFailed = 57, //
    ChoMatchComplete = 58,     //
    OsuMatchHasBeatmap = 59,
    OsuMatchSkipRequest = 60,
    ChoMatchSkip = 61,    //
    ChoUnauthorized = 62, // unused
    OsuChannelJoin = 63,
    ChoChannelJoinSuccess = 64, //
    ChoChannelInfo = 65,        //
    ChoChannelKick = 66,        //
    ChoChannelAutoJoin = 67,    //
    OsuBeatmapInfoRequest = 68,
    ChoBeatmapInfoReply = 69,
    OsuMatchTransferHost = 70,
    ChoPrivileges = 71,  //
    ChoFriendsList = 72, //
    OsuFriendAdd = 73,
    OsuFriendRemove = 74,
    ChoProtocolVersion = 75, //
    ChoMainMenuIcon = 76,    //
    OsuMatchChangeTeam = 77,
    OsuChannelPart = 78,
    OsuReceiveUpdates = 79,
    ChoMonitor = 80,            // unused //
    ChoMatchPlayerSkipped = 81, //
    OsuSetAwayMessage = 82,
    ChoUserPresence = 83, //
    OsuIrcOnly = 84,
    OsuUserStatsRequest = 85,
    ChoRestart = 86, //
    OsuMatchInvite = 87,
    ChoMatchInvite = 88,    //
    ChoChannelInfoEnd = 89, //
    OsuMatchChangePassword = 90,
    ChoMatchChangePassword = 91, //
    ChoSilenceEnd = 92,          //
    OsuTournamentMatchInfoRequest = 93,
    ChoUserSilenced = 94,       //
    ChoUserPresenceSingle = 95, //
    ChoUserPresenceBundle = 96, //
    OsuUserPresenceRequest = 97,
    OsuUserPresenceRequestAll = 98,
    OsuToggleBlockNonFriendDms = 99,
    ChoUserDmBlocked = 100,          //
    ChoTargetIsSilenced = 101,       //
    ChoVersionUpdateForced = 102,    //
    ChoSwitchServer = 103,           //
    ChoAccountRestricted = 104,      //
    ChoRtx = 105,                    // unused //
    ChoMatchAbort = 106,             //
    ChoSwitchTournamentServer = 107, //
    OsuTournamentJoinMatchChannel = 108,
    OsuTournamentLeaveMatchChannel = 109,
}

impl Serialize for Packets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i16(*self as i16)
    }
}

pub trait _Packet {}

pub struct ChoUserId {
    // 5
    pub user_id: i32,
}

pub struct ChoSendMessage {
    // 7 (why is it 7???)
    sender: String,
    text: String,
    recipient: String,
    sender_id: i32,
}

pub struct ChoPong {} // 8

pub struct ChoHandleIrcChangeUsername {
    // 9
    pub old: String,
    pub new: String,
}

pub struct ChoUserStats {
    // 11
    pub user_id: i32,
    pub action: u8,
    pub info_text: String,
    pub map_md5: String,
    pub mods: i32,
    pub mode: u8,
    pub map_id: i32,
    pub ranked_score: i64,
    pub accuracy: f32,
    pub play_count: i32,
    pub total_score: i64,
    pub global_rank: i32,
    pub pp: i16,
}

pub struct ChoUserLogout {
    // 12
    pub user_id: i32,
    pub u8term: u8, // req 0 byte at the end?
}

pub struct ChoSpectatorJoined {
    // 13
    pub user_id: i32,
}

pub struct ChoSpectatorLeft {
    // 14
    pub user_id: i32,
}

pub struct ChoSpectateFrames {
    // 15
    pub frames: Vec<u8>, // bytes!
}

pub struct ChoVersionUpdate {} // 19

pub struct ChoSpectatorCantSpectate {
    // 22
    pub user_id: i32,
}

pub struct ChoGetAttention {} // 23

pub struct ChoNotification {
    // 24
    pub msg: String,
}

// pub struct ChoUpdateMatch { //26

// }

pub struct ChoDisposeMatch {
    // 28
    pub match_id: i32,
}

pub struct ChoToggleBlockNonFriendDms {} // 34

pub struct ChoMatchJoinFail {} // 37

pub struct ChoFellowSpectatorJoined {
    // 42
    pub user_id: i32,
}

pub struct ChoFellowSpectatorLeft {
    // 43
    pub user_id: i32,
}

pub struct ChoMatchTransferHost {} // 50

pub struct ChoMatchAllPlayersLoaded {} // 53

pub struct ChoMatchPlayerFailed {
    // 57
    pub slot_id: i32,
}

pub struct ChoMatchComplete {} // 58

pub struct ChoMatchSkip {} // 61

pub struct ChoChannelJoinSuccess {
    // 64
    pub name: String,
}

pub struct ChoChannelInfo {
    //65
    pub name: String,
    pub topic: String,
    pub player_count: i32,
}

pub struct ChoChannelKick {
    //66
    pub name: String,
}

pub struct ChoChannelAutoJoin {
    //67
    pub name: String,
    pub topic: String,
    pub player_count: i32,
}

pub struct ChoPrivileges {
    //71
    pub privileges: i32,
}

pub struct ChoFriendsList {
    //72
    pub friends: Vec<i32>, // list of ids
}

pub struct ChoProtocolVersion {
    //75
    pub version: i32,
}

pub struct ChoMainMenuIcon {
    //76
    pub icon_url: String,
    pub onclick_url: String, // there is a | delimiter
}

pub struct ChoMonitor {} // 80

pub struct ChoMatchPlayerSkipped {
    // 81
    pub user_id: i32,
}

pub struct ChoUserPresence {
    //83
    pub user_id: i32,
    pub name: String,
    pub utc_offset: u8,
    pub country_code: u8,
    pub bancho_privileges: u8,
    pub longitude: f32,
    pub latitude: f32,
    pub global_rank: i32,
}

pub struct ChoRestart {
    // 86
    pub ms_delay: i32,
}

pub struct ChoMatchInvite {
    // 88
    pub player_name: String,
    pub msg: String,
    pub recipient: String,
    pub sender_id: i32,
}

pub struct ChoChannelInfoEnd {} // 89

pub struct ChoMatchChangePassword {
    // 91
    pub new_password: String,
}

pub struct ChoSilenceEnd {
    // 92
    pub delta: i32,
}

pub struct ChoUserSilenced {
    // 94
    pub user_id: i32,
}

pub struct ChoUserPresenceSingle {
    // 95
    pub user_id: i32,
}

pub struct ChoUserPresenceBundle {
    // 96
    pub user_ids: Vec<i32>,
}

pub struct ChoUserDmBlocked {
    //100
    pub player_name: String,
    pub msg: String,
    pub recipient: String,
    pub sender_id: i32,
}

pub struct ChoTargetIsSilenced {
    //101
    pub player_name: String,
    pub msg: String,
    pub recipient: String,
    pub sender_id: i32,
}

pub struct ChoVersionUpdateForced {} //102

pub struct ChoSwitchServer {
    // 103
    pub t: i32, //what is t?
}

pub struct ChoAccountRestricted {} // 104

pub struct ChoRtx {
    // 105
    pub msg: String,
}

pub struct ChoMatchAbort {} // 106

pub struct ChoSwitchTournamentServer {
    // 107
    pub ip: String,
}

// god forgive
impl _Packet for ChoUserId {}
impl _Packet for ChoSendMessage {}
impl _Packet for ChoPong {}
impl _Packet for ChoHandleIrcChangeUsername {}
impl _Packet for ChoUserStats {}
impl _Packet for ChoUserLogout {}
impl _Packet for ChoSpectatorJoined {}
impl _Packet for ChoSpectatorLeft {}
impl _Packet for ChoSpectateFrames {}
impl _Packet for ChoVersionUpdate {}
impl _Packet for ChoSpectatorCantSpectate {}
impl _Packet for ChoGetAttention {}
impl _Packet for ChoNotification {}
impl _Packet for ChoDisposeMatch {}
impl _Packet for ChoToggleBlockNonFriendDms {}
impl _Packet for ChoMatchJoinFail {}
impl _Packet for ChoFellowSpectatorJoined {}
impl _Packet for ChoFellowSpectatorLeft {}
impl _Packet for ChoMatchTransferHost {}
impl _Packet for ChoMatchAllPlayersLoaded {}
impl _Packet for ChoMatchPlayerFailed {}
impl _Packet for ChoMatchComplete {}
impl _Packet for ChoMatchSkip {}
impl _Packet for ChoChannelJoinSuccess {}
impl _Packet for ChoChannelInfo {}
impl _Packet for ChoChannelKick {}
impl _Packet for ChoChannelAutoJoin {}
impl _Packet for ChoPrivileges {}
impl _Packet for ChoFriendsList {}
impl _Packet for ChoProtocolVersion {}
impl _Packet for ChoMainMenuIcon {}
impl _Packet for ChoMonitor {}
impl _Packet for ChoMatchPlayerSkipped {}
impl _Packet for ChoUserPresence {}
impl _Packet for ChoRestart {}
impl _Packet for ChoMatchInvite {}
impl _Packet for ChoChannelInfoEnd {}
impl _Packet for ChoMatchChangePassword {}
impl _Packet for ChoSilenceEnd {}
impl _Packet for ChoUserSilenced {}
impl _Packet for ChoUserPresenceSingle {}
impl _Packet for ChoUserPresenceBundle {}
impl _Packet for ChoUserDmBlocked {}
impl _Packet for ChoTargetIsSilenced {}
impl _Packet for ChoVersionUpdateForced {}
impl _Packet for ChoSwitchServer {}
impl _Packet for ChoAccountRestricted {}
impl _Packet for ChoRtx {}
impl _Packet for ChoMatchAbort {}
impl _Packet for ChoSwitchTournamentServer {}

// id 5
impl ChoUserId {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoUserId, self.user_id))
    }
}

// id 7
impl ChoSendMessage {
    pub async fn new<T, U>(sender: T, text: T, recipient: T, sender_id: U) -> Self
    where
        T: Into<String>,
        U: Into<i32>,
    {
        Self {
            sender: sender.into(),
            text: text.into(),
            recipient: recipient.into(),
            sender_id: sender_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoSendMessage,
            &self.sender,
            &self.text,
            &self.recipient,
            &self.sender_id
        ))
    }
}

// id 8
impl ChoPong {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoPong);
        Ok(w.serialise())
    }
}

// id 9
impl ChoHandleIrcChangeUsername {
    pub async fn new<T>(old: T, new: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            old: old.into(),
            new: new.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoHandleIrcChangeUsername,
            format!("{}>>>>{}", self.old, self.new)
        ))
    }
}

// anti peppy
// id 11

impl ChoUserStats {
    /// user_id: i32,
    /// action: u8,
    /// info_text: String,
    /// map_md5: String,
    /// mods: i32
    /// mode: u8,
    /// map_id: i32,
    /// ranked_score: i64,
    /// accuracy: f32,
    /// play_count: i32,
    /// total_score: i64,
    /// global_rank: i32,
    /// pp: i16
    pub async fn new<T, U, V, W, X, Y>(
        user_id: T,
        action: U,
        info_text: V,
        map_md5: V,
        mods: T,
        mode: U,
        map_id: T,
        ranked_score: Y,
        accuracy: X,
        play_count: T,
        total_score: Y,
        global_rank: T,
        pp: W,
    ) -> Self
    where
        T: Into<i32>,
        U: Into<u8>,
        V: Into<String>,
        W: Into<i16>,
        X: Into<f32>,
        Y: Into<i64>,
    {
        Self {
            user_id: user_id.into(),
            action: action.into(),
            info_text: info_text.into(),
            map_md5: map_md5.into(),
            mods: mods.into(),
            mode: mode.into(),
            map_id: map_id.into(),
            ranked_score: ranked_score.into(),
            accuracy: accuracy.into(),
            play_count: play_count.into(),
            total_score: total_score.into(),
            global_rank: global_rank.into(),
            pp: pp.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoUserStats,
            &self.user_id,
            &self.action,
            &self.info_text,
            &self.map_md5,
            &self.mods,
            &self.mode,
            &self.map_id,
            &self.ranked_score,
            &self.accuracy,
            &self.play_count,
            &self.total_score,
            &self.global_rank,
            &self.pp
        ))
    }
}

// id 12
impl ChoUserLogout {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
            u8term: 0,
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoUserLogout, self.user_id, 0))
    }
}

// id 13
impl ChoSpectatorJoined {
    pub async fn new<T, U>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoSpectatorJoined, self.user_id))
    }
}

// id 14
impl ChoSpectatorLeft {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoSpectatorLeft, self.user_id))
    }
}

// id 15
impl ChoSpectateFrames {
    pub async fn new<T>(frames: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        Self {
            frames: frames.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoSpectateFrames, self.frames))
    }
}

// id 19
impl ChoVersionUpdate {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoVersionUpdate);
        Ok(w.serialise())
    }
}

// id 22
impl ChoSpectatorCantSpectate {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoSpectatorCantSpectate,
            self.user_id
        ))
    }
}

// id 23
impl ChoGetAttention {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoGetAttention);
        Ok(w.serialise())
    }
}

// id 24
impl ChoNotification {
    pub async fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { msg: text.into() }
    }
    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoNotification, &self.msg))
    }
}

// id 28
impl ChoDisposeMatch {
    pub async fn new<T>(match_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            match_id: match_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoDisposeMatch, self.match_id))
    }
}

// id 34
impl ChoToggleBlockNonFriendDms {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoToggleBlockNonFriendDms);
        Ok(w.serialise())
    }
}

// id 37
impl ChoMatchJoinFail {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoMatchJoinFail);
        Ok(w.serialise())
    }
}

// id 42
impl ChoFellowSpectatorJoined {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoFellowSpectatorJoined,
            self.user_id
        ))
    }
}

// id 43
impl ChoFellowSpectatorLeft {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoFellowSpectatorLeft, self.user_id))
    }
}

// all packets in gulag that take in a mutiplayer match have been ommited
// beacuse we dont have that type and i cant be fucked to implement it

// id 53
impl ChoMatchAllPlayersLoaded {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoMatchAllPlayersLoaded);
        Ok(w.serialise())
    }
}

// id 57
impl ChoMatchPlayerFailed {
    pub async fn new<T>(slot_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            slot_id: slot_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoMatchPlayerFailed, self.slot_id))
    }
}

// id 58
impl ChoMatchComplete {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoMatchComplete);
        Ok(w.serialise())
    }
}

// id 61
impl ChoMatchSkip {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoMatchSkip);
        Ok(w.serialise())
    }
}

// id 64
impl ChoChannelJoinSuccess {
    pub async fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoChannelJoinSuccess, &self.name))
    }
}

// id 65
impl ChoChannelInfo {
    pub async fn new<T, U>(name: T, topic: T, player_count: U) -> Self
    where
        T: Into<String>,
        U: Into<i32>,
    {
        Self {
            name: name.into(),
            topic: topic.into(),
            player_count: player_count.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoChannelInfo,
            &self.name,
            &self.topic,
            self.player_count
        ))
    }
}

// id 66
impl ChoChannelKick {
    pub async fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoChannelKick, &self.name))
    }
}

// id 67
impl ChoChannelAutoJoin {
    pub async fn new<T, U>(name: T, topic: T, player_count: U) -> Self
    where
        T: Into<String>,
        U: Into<i32>,
    {
        Self {
            name: name.into(),
            topic: topic.into(),
            player_count: player_count.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoChannelAutoJoin,
            &self.name,
            &self.topic,
            self.player_count
        ))
    }
}

// id 71
impl ChoPrivileges {
    pub async fn new<T>(privileges: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            privileges: privileges.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoPrivileges, self.privileges))
    }
}

// id 72
impl ChoFriendsList {
    pub async fn new<T>(friends: T) -> Self
    where
        T: Into<Vec<i32>>,
    {
        Self {
            friends: friends.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoFriendsList, self.friends))
    }
}

// id 75
impl ChoProtocolVersion {
    pub async fn new<T>(version: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            version: version.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoProtocolVersion, self.version))
    }
}

// do it
// tell me when you d

// id 76
impl ChoMainMenuIcon {
    pub async fn new<T>(icon_url: T, onclick_url: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            icon_url: icon_url.into(),
            onclick_url: onclick_url.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoMainMenuIcon,
            format!("{}|{}", &self.icon_url, &self.onclick_url)
        ))
    }
}

// id 80
impl ChoMonitor {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoMonitor);
        Ok(w.serialise())
    }
}

// id 81
impl ChoMatchPlayerSkipped {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoMatchPlayerSkipped, self.user_id))
    }
}

// id 83
impl ChoUserPresence {
    pub async fn new<T, U, V, W>(
        user_id: T,
        name: U,
        utc_offset: V,
        country_code: V,
        bancho_privileges: V,
        longitude: W,
        latitude: W,
        global_rank: T,
    ) -> Self
    where
        T: Into<i32>,
        U: Into<String>,
        V: Into<u8>,
        W: Into<f32>,
    {
        Self {
            user_id: user_id.into(),
            name: name.into(),
            utc_offset: utc_offset.into(),
            country_code: country_code.into(),
            bancho_privileges: bancho_privileges.into(),
            longitude: longitude.into(),
            latitude: latitude.into(),
            global_rank: global_rank.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoUserPresence,
            self.user_id,
            &self.name,
            self.utc_offset,
            self.country_code,
            self.bancho_privileges,
            self.longitude,
            self.latitude,
            self.global_rank
        ))
    }
}

// id 86
impl ChoRestart {
    pub async fn new<T>(ms_delay: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            ms_delay: ms_delay.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoRestart, self.ms_delay))
    }
}

// id 88
impl ChoMatchInvite {
    pub async fn new<T, U>(player_name: T, msg: T, recipient: T, sender_id: U) -> Self
    where
        T: Into<String>,
        U: Into<i32>,
    {
        Self {
            player_name: player_name.into(),
            msg: msg.into(),
            recipient: recipient.into(),
            sender_id: sender_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoMatchInvite,
            &self.player_name,
            &self.msg,
            &self.recipient,
            self.sender_id
        ))
    }
}

// id 89
impl ChoChannelInfoEnd {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoChannelInfoEnd);
        Ok(w.serialise())
    }
}

// id 91
impl ChoMatchChangePassword {
    pub async fn new<T>(new_password: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            new_password: new_password.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoMatchChangePassword,
            &self.new_password
        ))
    }
}

// id 92
impl ChoSilenceEnd {
    pub async fn new<T>(delta: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            delta: delta.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoSilenceEnd, self.delta))
    }
}

// id 94
impl ChoUserSilenced {
    pub async fn new<T>(user_id: T, silence_end: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoUserSilenced, self.user_id))
    }
}

// id 95
impl ChoUserPresenceSingle {
    pub async fn new<T>(user_id: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            user_id: user_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoUserPresenceSingle, self.user_id))
    }
}

impl ChoUserPresenceBundle {
    pub async fn new<T>(user_ids: T) -> Self
    where
        T: Into<Vec<i32>>,
    {
        Self {
            user_ids: user_ids.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoUserPresenceBundle, self.user_ids))
    }
}

impl ChoUserDmBlocked {
    pub async fn new<T, U>(player_name: T, msg: T, recipient: T, sender_id: U) -> Self
    where
        T: Into<String>,
        U: Into<i32>,
    {
        Self {
            player_name: player_name.into(),
            msg: msg.into(),
            recipient: recipient.into(),
            sender_id: sender_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoUserDmBlocked,
            &self.player_name,
            &self.msg,
            &self.recipient,
            self.sender_id
        ))
    }
}

impl ChoTargetIsSilenced {
    pub async fn new<T, U>(player_name: T, msg: T, recipient: T, sender_id: U) -> Self
    where
        T: Into<String>,
        U: Into<i32>,
    {
        Self {
            player_name: player_name.into(),
            msg: msg.into(),
            recipient: recipient.into(),
            sender_id: sender_id.into(),
        }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(
            Packets::ChoTargetIsSilenced,
            &self.player_name,
            &self.msg,
            &self.recipient,
            self.sender_id
        ))
    }
}

impl ChoVersionUpdateForced {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoVersionUpdateForced);
        Ok(w.serialise())
    }
}

impl ChoSwitchServer {
    pub async fn new<T>(t: T) -> Self
    where
        T: Into<i32>,
    {
        Self { t: t.into() }
    }
}

impl ChoAccountRestricted {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoAccountRestricted);
        Ok(w.serialise())
    }
}

impl ChoRtx {
    pub async fn new<T>(msg: T) -> Self
    where
        T: Into<String>,
    {
        Self { msg: msg.into() }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoRtx, &self.msg))
    }
}

impl ChoMatchAbort {
    pub async fn write() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut w = PacketWriter::new(Packets::ChoMatchAbort);
        Ok(w.serialise())
    }
}

impl ChoSwitchTournamentServer {
    pub async fn new<T>(ip: T) -> Self
    where
        T: Into<String>,
    {
        Self { ip: ip.into() }
    }

    pub async fn write(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(write_packet!(Packets::ChoSwitchTournamentServer, &self.ip))
    }
}
