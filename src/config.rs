use std::path::PathBuf;

use crate::constants::{
    DIST_CNT, USER_DIR_ALL, USER_DIR_CONT, USER_DIR_EAST, USER_DIR_NORTH, USER_DIR_NORTH_EAST,
    USER_DIR_NORTH_WEST, USER_DIR_REPEAT, USER_DIR_SOUTH, USER_DIR_SOUTH_EAST, USER_DIR_SOUTH_WEST,
    USER_DIR_WEST, USER_DIST_MAX, USER_DIST_MIN, USER_MOD_ALL, USER_MOD_ALTGR, USER_MOD_BASIC,
    USER_MOD_SHIFT,
};
use crate::error::{KwpError, Result};
use crate::keymap::Layer;

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) output_file: Option<PathBuf>,
    pub(crate) user_mod_basic: i32,
    pub(crate) user_mod_shift: i32,
    pub(crate) user_mod_altgr: i32,
    pub(crate) user_mod_all: bool,
    pub(crate) user_dir_south_west: i32,
    pub(crate) user_dir_south: i32,
    pub(crate) user_dir_south_east: i32,
    pub(crate) user_dir_west: i32,
    pub(crate) user_dir_repeat: i32,
    pub(crate) user_dir_east: i32,
    pub(crate) user_dir_north_west: i32,
    pub(crate) user_dir_north: i32,
    pub(crate) user_dir_north_east: i32,
    pub(crate) user_dir_cont: bool,
    pub(crate) user_dir_all: bool,
    pub(crate) user_dist_min: usize,
    pub(crate) user_dist_max: usize,
    pub(crate) compat_order: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_file: None,
            user_mod_basic: USER_MOD_BASIC,
            user_mod_shift: USER_MOD_SHIFT,
            user_mod_altgr: USER_MOD_ALTGR,
            user_mod_all: USER_MOD_ALL,
            user_dir_south_west: USER_DIR_SOUTH_WEST,
            user_dir_south: USER_DIR_SOUTH,
            user_dir_south_east: USER_DIR_SOUTH_EAST,
            user_dir_west: USER_DIR_WEST,
            user_dir_repeat: USER_DIR_REPEAT,
            user_dir_east: USER_DIR_EAST,
            user_dir_north_west: USER_DIR_NORTH_WEST,
            user_dir_north: USER_DIR_NORTH,
            user_dir_north_east: USER_DIR_NORTH_EAST,
            user_dir_cont: USER_DIR_CONT,
            user_dir_all: USER_DIR_ALL,
            user_dist_min: USER_DIST_MIN,
            user_dist_max: USER_DIST_MAX,
            compat_order: false,
        }
    }
}

impl Config {
    pub(crate) fn apply_shortcuts_and_validate(&mut self) -> Result<()> {
        if self.user_dist_min < 1 {
            return Err(KwpError::Message(
                "Keywalk distance minimum can not be smaller than 1".to_string(),
            ));
        }

        if self.user_dist_max < 1 {
            return Err(KwpError::Message(
                "Keywalk distance maximum can not be smaller than 1".to_string(),
            ));
        }

        if self.user_dist_min > DIST_CNT {
            return Err(KwpError::Message(format!(
                "Keywalk distance minimum can not be greater than {DIST_CNT}"
            )));
        }

        if self.user_dist_max > DIST_CNT {
            return Err(KwpError::Message(format!(
                "Keywalk distance maximum can not be greater than {DIST_CNT}"
            )));
        }

        if self.user_dist_min > self.user_dist_max {
            return Err(KwpError::Message(
                "Keywalk distance minimum can not be greater than maximum".to_string(),
            ));
        }

        if self.user_mod_all {
            self.user_mod_basic = 1;
            self.user_mod_shift = 1;
            self.user_mod_altgr = 1;
        }

        if self.user_dir_cont {
            self.user_dir_south_west = 1;
            self.user_dir_south = 1;
            self.user_dir_south_east = 0;
            self.user_dir_west = 1;
            self.user_dir_repeat = 1;
            self.user_dir_east = 1;
            self.user_dir_north_west = 0;
            self.user_dir_north = 1;
            self.user_dir_north_east = 1;
        }

        if self.user_dir_all {
            self.user_dir_south_west = 1;
            self.user_dir_south = 1;
            self.user_dir_south_east = 1;
            self.user_dir_west = 1;
            self.user_dir_repeat = 1;
            self.user_dir_east = 1;
            self.user_dir_north_west = 1;
            self.user_dir_north = 1;
            self.user_dir_north_east = 1;
        }

        for (name, value) in [
            ("keyboard-basic", self.user_mod_basic),
            ("keyboard-shift", self.user_mod_shift),
            ("keyboard-altgr", self.user_mod_altgr),
            ("keywalk-south-west", self.user_dir_south_west),
            ("keywalk-south", self.user_dir_south),
            ("keywalk-south-east", self.user_dir_south_east),
            ("keywalk-west", self.user_dir_west),
            ("keywalk-repeat", self.user_dir_repeat),
            ("keywalk-east", self.user_dir_east),
            ("keywalk-north-west", self.user_dir_north_west),
            ("keywalk-north", self.user_dir_north),
            ("keywalk-north-east", self.user_dir_north_east),
        ] {
            if value < 0 {
                return Err(KwpError::Message(format!("{name} can not be negative")));
            }
        }

        Ok(())
    }

    pub(crate) fn dist_count(&self) -> usize {
        1 + (self.user_dist_max - self.user_dist_min)
    }

    pub(crate) fn modifier_slots(&self) -> Vec<Option<Layer>> {
        let mod_count = self.mod_count();
        let mut slots = Vec::with_capacity(mod_count);

        if self.user_mod_basic == 1 {
            slots.push(Some(Layer::Basic));
        }
        if self.user_mod_shift == 1 {
            slots.push(Some(Layer::Shift));
        }
        if self.user_mod_altgr == 1 {
            slots.push(Some(Layer::AltGr));
        }

        slots.resize(mod_count, None);
        slots
    }

    pub(crate) fn direction_slots(&self) -> Vec<Option<(i32, i32)>> {
        let dir_count = self.dir_count();
        let mut slots = Vec::with_capacity(dir_count);

        if self.user_dir_south_west == 1 {
            slots.push(Some((-1, 1)));
        }
        if self.user_dir_south == 1 {
            slots.push(Some((0, 1)));
        }
        if self.user_dir_south_east == 1 {
            slots.push(Some((1, 1)));
        }
        if self.user_dir_west == 1 {
            slots.push(Some((-1, 0)));
        }
        if self.user_dir_repeat == 1 {
            slots.push(Some((0, 0)));
        }
        if self.user_dir_east == 1 {
            slots.push(Some((1, 0)));
        }
        if self.user_dir_north_west == 1 {
            slots.push(Some((-1, -1)));
        }
        if self.user_dir_north == 1 {
            slots.push(Some((0, -1)));
        }
        if self.user_dir_north_east == 1 {
            slots.push(Some((1, -1)));
        }

        slots.resize(dir_count, None);
        slots
    }

    pub(crate) fn mod_count(&self) -> usize {
        (self.user_mod_basic + self.user_mod_shift + self.user_mod_altgr) as usize
    }

    pub(crate) fn dir_count(&self) -> usize {
        (self.user_dir_south_west
            + self.user_dir_south
            + self.user_dir_south_east
            + self.user_dir_west
            + self.user_dir_repeat
            + self.user_dir_east
            + self.user_dir_north_west
            + self.user_dir_north
            + self.user_dir_north_east) as usize
    }
}
