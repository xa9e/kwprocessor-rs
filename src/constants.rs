pub(crate) const VERSION_BIN: u32 = 100;

pub(crate) const DIST_CNT: usize = 16;
pub(crate) const MOD_CNT: usize = 3;
pub(crate) const DIR_CNT: usize = 9;

pub(crate) const KEYMAP_WIDTH: usize = 14;
pub(crate) const KEYMAP_HEIGHT: usize = 4;
pub(crate) const KEYMAP_ENTRIES: usize = KEYMAP_WIDTH * KEYMAP_HEIGHT;

pub(crate) const ROUTE_LENGTH_MIN: usize = 1;
pub(crate) const ROUTE_LENGTH_MAX: usize = 32;
pub(crate) const ROUTE_REPEAT_MIN: u8 = 1;
pub(crate) const ROUTE_REPEAT_MAX: u8 = 16;

pub(crate) const USER_MOD_BASIC: i32 = 1;
pub(crate) const USER_MOD_SHIFT: i32 = 0;
pub(crate) const USER_MOD_ALTGR: i32 = 0;
pub(crate) const USER_MOD_ALL: bool = false;

pub(crate) const USER_DIR_SOUTH_WEST: i32 = 0;
pub(crate) const USER_DIR_SOUTH: i32 = 1;
pub(crate) const USER_DIR_SOUTH_EAST: i32 = 0;
pub(crate) const USER_DIR_WEST: i32 = 1;
pub(crate) const USER_DIR_REPEAT: i32 = 0;
pub(crate) const USER_DIR_EAST: i32 = 1;
pub(crate) const USER_DIR_NORTH_WEST: i32 = 0;
pub(crate) const USER_DIR_NORTH: i32 = 1;
pub(crate) const USER_DIR_NORTH_EAST: i32 = 0;
pub(crate) const USER_DIR_CONT: bool = false;
pub(crate) const USER_DIR_ALL: bool = false;

pub(crate) const USER_DIST_MIN: usize = 1;
pub(crate) const USER_DIST_MAX: usize = 1;

pub(crate) const INVALID_CODE: u32 = u32::MAX;
pub(crate) const INVALID_IDX: usize = usize::MAX;
pub(crate) const UNSET_INDEX: usize = usize::MAX;
pub(crate) const UNICODE_LIMIT: usize = 0x11_0000;

pub(crate) const MAX_PASSWORD_CHARS: usize = 1 + ROUTE_LENGTH_MAX * ROUTE_REPEAT_MAX as usize;
pub(crate) const MAX_PASSWORD_BYTES: usize = MAX_PASSWORD_CHARS * 4;
pub(crate) const OUTPUT_BUFFER_CAPACITY: usize = 1 << 20;
