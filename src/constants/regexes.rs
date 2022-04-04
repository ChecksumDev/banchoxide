pub static OSU_VERSION: &str =
    "^b(?P<date>\\d{8})(?:\\.(?P<revision>\\d))?(?P<stream>beta|cuttingedge|dev|tourney)?$";
pub static USERNAME: &str = "^[\\w \\[\\]-]{2,15}$";
pub static EMAIL: &str = "^[^@\\s]{1,200}@[^@\\s\\.]{1,30}(?:\\.[^@\\.\\s]{2,24})+$";
pub static SCALED_DURATION: &str = "^(?P<duration>\\d{1,6})(?P<scale>s|m|h|d|w)$";
pub static TOURNEY_MATCHNAME: &str =
    "^(?P<name>[a-zA-Z0-9_ ]+): \\((?P<T1>[a-zA-Z0-9_ ]+)\\) vs\\.? \\((?P<T2>[a-zA-Z0-9_ ]+)\\)$";
pub static MAPPOOL_PICK: &str = "^([a-zA-Z]+)([0-9]+)$";
pub static BEST_OF: &str = "^(?:bo)?(\\d{1,2})$";
