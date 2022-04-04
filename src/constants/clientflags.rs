use bitflags::bitflags;

bitflags! {
    pub struct ClientFlags: u32 {
        const CLEAN = 0;
        const SPEED_HACK_DETECTED = 1 << 1;
        const INCORRECT_MOD_VALUE = 1 << 2;
        const MULTIPLE_OSU_CLIENTS = 1 << 3;
        const CHECKSUM_FAILURE = 1 << 4;
        const FLASHLIGHT_CHECKSUM_INCORRECT = 1 << 5;
        const OSU_EXECUTABLE_CHECKSUM = 1 << 6;
        const MISSING_PROCESSES_IN_LIST = 1 << 7;
        const FLASHLIGHT_IMAGE_HACK = 1 << 8;
        const SPINNER_HACK = 1 << 9;
        const TRANSPARENT_WINDOW = 1 << 10;
        const FAST_PRESS = 1 << 11;
        const RAW_MOUSE_DISCREPANCY = 1 << 12;
        const RAW_KEYBOARD_DISCREPANCY = 1 << 13;
        const RUN_WITH_LD_FLAG = 1 << 14;
        const CONSOLE_OPEN = 1 << 15;
        const EXTRA_THREADS = 1 << 16;
        const HQ_ASSEMBLY = 1 << 17;
        const HQ_FILE = 1 << 18;
        const REGISTRY_EDITS = 1 << 19;
        const SDL2_LIBRARY = 1 << 20;
        const OPENSSL_LIBRARY = 1 << 21;
        const AQN_MENU_SAMPLE = 1 << 22;
    }
}

#[test]
fn add_all_flags() {
    let mut flags = ClientFlags::empty();
    flags.insert(ClientFlags::CLEAN);
    flags.insert(ClientFlags::SPEED_HACK_DETECTED);
    flags.insert(ClientFlags::INCORRECT_MOD_VALUE);
    flags.insert(ClientFlags::MULTIPLE_OSU_CLIENTS);
    flags.insert(ClientFlags::CHECKSUM_FAILURE);
    flags.insert(ClientFlags::FLASHLIGHT_CHECKSUM_INCORRECT);
    flags.insert(ClientFlags::OSU_EXECUTABLE_CHECKSUM);
    flags.insert(ClientFlags::MISSING_PROCESSES_IN_LIST);
    flags.insert(ClientFlags::FLASHLIGHT_IMAGE_HACK);
    flags.insert(ClientFlags::SPINNER_HACK);
    flags.insert(ClientFlags::TRANSPARENT_WINDOW);
    flags.insert(ClientFlags::FAST_PRESS);
    flags.insert(ClientFlags::RAW_MOUSE_DISCREPANCY);
    flags.insert(ClientFlags::RAW_KEYBOARD_DISCREPANCY);
    flags.insert(ClientFlags::RUN_WITH_LD_FLAG);
    flags.insert(ClientFlags::CONSOLE_OPEN);
    flags.insert(ClientFlags::EXTRA_THREADS);
    flags.insert(ClientFlags::HQ_ASSEMBLY);
    flags.insert(ClientFlags::HQ_FILE);
    flags.insert(ClientFlags::REGISTRY_EDITS);
    flags.insert(ClientFlags::SDL2_LIBRARY);
    flags.insert(ClientFlags::OPENSSL_LIBRARY);
    flags.insert(ClientFlags::AQN_MENU_SAMPLE);
    assert_eq!(flags.bits(), 0x7fffffff);
}
