use std::fmt::{Display, Formatter};

use bitflags::bitflags;

use super::mods::Mods;

static GAMEMODE_REPR_LIST: [&str; 10] = [
    "vn!std", "vn!taiko", "vn!catch", "vn!mania", "rx!std", "rx!taiko", "rx!catch", "rx!mania",
    "ap!std", "ap!taiko",
];

bitflags! {
    pub struct GameMode: u32 {
        const VANILLA_OSU = 0;
        const VANILLA_TAIKO = 1;
        const VANILLA_CATCH = 2;
        const VANILLA_MANIA = 3;

        const RELAX_OSU = 4;
        const RELAX_TAIKO = 5;
        const RELAX_CATCH = 6;
        const RELAX_MANIA = 7;

        const AUTOPILOT_OSU = 8; // smile
        const AUTOPILOT_TAIKO = 9;
        const AUTOPILOT_CATCH = 10;
        const AUTOPILOT_MANIA = 11;
    }
}

impl Display for GameMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = GAMEMODE_REPR_LIST[self.bits() as usize];
        write!(f, "{}", repr)
    }
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::VANILLA_OSU
    }
}

impl GameMode {
    pub fn from_params(mode_vn: u32, mods: u32) -> Self {
        let mut mode = mode_vn;

        if mods & Mods::AUTOPILOT.bits() != 0 {
            mode += Self::AUTOPILOT_OSU.bits();
        } else if mods & Mods::RELAX.bits() != 0 {
            mode += Self::RELAX_OSU.bits();
        }

        Self::from_bits_truncate(mode)
    }

    pub fn as_vanilla(self) -> u32 {
        if self.contains(Self::AUTOPILOT_OSU) {
            self.bits() - Self::AUTOPILOT_OSU.bits()
        } else if self.contains(Self::RELAX_OSU) {
            self.bits() - Self::RELAX_OSU.bits()
        } else {
            self.bits()
        }
    }

    pub fn as_string(self) -> &'static str {
        GAMEMODE_REPR_LIST[self.bits() as usize]
    }
}
