use bitflags::bitflags;

bitflags! {
    pub struct Privileges: u32 {
        const NORMAL = 1 << 0;
        const VERIFIED = 1 << 1;
        const WHITELISTED = 1 << 2;
        const SUPPORTER = 1 << 4;
        const PREMIUM = 1 << 5;
        const ALUMNI = 1 << 7;
        const TOURNAMENT = 1 << 10;
        const NOMINATOR = 1 << 11;
        const MODERATOR = 1 << 12;
        const ADMINISTRATOR = 1 << 13;
        const DEVELOPER = 1 << 14;
        const DONATOR = 1 << 4 | 1 << 5; // SUPPORTER | PREMIUM
        const STAFF = 1 << 12 | 1 << 13 | 1 << 14; // MODERATOR | ADMINISTRATOR | DEVELOPER
    }
}

bitflags! {
    pub struct ClientPrivileges: u32 {
        const PLAYER = 1 << 0;
        const MODERATOR = 1 << 1;
        const SUPPORTER = 1 << 2;
        const OWNER = 1 << 3;
        const DEVELOPER = 1 << 4;
        const TOURNAMENT = 1 << 5;
    }
}

bitflags! {
    pub struct ClanPrivileges: u32 {
        const MEMBER = 1;
        const OFFICER = 2;
        const OWNER = 3;
    }
}
