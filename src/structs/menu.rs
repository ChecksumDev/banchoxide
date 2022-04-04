use std::collections::HashMap;

use bitflags::bitflags;

bitflags! {
    pub struct MenuCommands : u32 {
        const RESET = 0;
        const BACK = 1;
        const ADVANCE = 2;
        const EXECUTE = 3;
    }
}

struct Menu {
    name: String,
    options: HashMap<u32, (MenuCommands, Option<Menu>)>,
}

struct MenuFunction {
    name: String,
    //callback: fn(Player),
}

fn menu_keygen() -> u32 {
    rand::random::<u32>()
}