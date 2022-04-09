pub const OSU_VERSION: &str =
    "^b(?P<date>\\d{8})(?:\\.(?P<revision>\\d))?(?P<stream>beta|cuttingedge|dev|tourney)?$";
pub const USERNAME: &str = "^[\\w \\[\\]-]{2,15}$";
pub const EMAIL: &str = "^[^@\\s]{1,200}@[^@\\s\\.]{1,30}(?:\\.[^@\\.\\s]{2,24})+$";
pub const SCALED_DURATION: &str = "^(?P<duration>\\d{1,6})(?P<scale>s|m|h|d|w)$";
pub const TOURNEY_MATCHNAME: &str =
    "^(?P<name>[a-zA-Z0-9_ ]+): \\((?P<T1>[a-zA-Z0-9_ ]+)\\) vs\\.? \\((?P<T2>[a-zA-Z0-9_ ]+)\\)$";
pub const MAPPOOL_PICK: &str = "^([a-zA-Z]+)([0-9]+)$";
pub const BEST_OF: &str = "^(?:bo)?(\\d{1,2})$";
