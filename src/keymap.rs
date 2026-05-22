use crate::constants::{INVALID_CODE, KEYMAP_ENTRIES, KEYMAP_HEIGHT, KEYMAP_WIDTH};

#[derive(Clone, Copy)]
pub(crate) struct Coord {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Clone, Copy)]
pub(crate) struct Flags {
    pub(crate) is_basic: bool,
    pub(crate) is_shift: bool,
    pub(crate) is_altgr: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum Layer {
    Basic,
    Shift,
    AltGr,
}

pub(crate) struct Keymaps {
    basic: [u32; KEYMAP_ENTRIES],
    shift: [u32; KEYMAP_ENTRIES],
    altgr: [u32; KEYMAP_ENTRIES],
}

impl Keymaps {
    pub(crate) fn new() -> Self {
        Self {
            basic: [INVALID_CODE; KEYMAP_ENTRIES],
            shift: [INVALID_CODE; KEYMAP_ENTRIES],
            altgr: [INVALID_CODE; KEYMAP_ENTRIES],
        }
    }

    #[inline(always)]
    pub(crate) fn idx(x: usize, y: usize) -> usize {
        x * KEYMAP_HEIGHT + y
    }

    pub(crate) fn set(&mut self, layer: Layer, x: usize, y: usize, code: u32) {
        let idx = Self::idx(x, y);
        match layer {
            Layer::Basic => self.basic[idx] = code,
            Layer::Shift => self.shift[idx] = code,
            Layer::AltGr => self.altgr[idx] = code,
        }
    }

    pub(crate) fn layer(&self, layer: Layer) -> &[u32; KEYMAP_ENTRIES] {
        match layer {
            Layer::Basic => &self.basic,
            Layer::Shift => &self.shift,
            Layer::AltGr => &self.altgr,
        }
    }

    #[inline(always)]
    pub(crate) fn char_at(&self, layer: Layer, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 {
            return INVALID_CODE;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= KEYMAP_WIDTH || y >= KEYMAP_HEIGHT {
            return INVALID_CODE;
        }

        self.layer(layer)[Self::idx(x, y)]
    }

    pub(crate) fn find_coord_in(&self, layer: Layer, code: u32) -> Option<Coord> {
        let map = self.layer(layer);

        for x in 0..KEYMAP_WIDTH {
            for y in 0..KEYMAP_HEIGHT {
                if map[Self::idx(x, y)] == code {
                    return Some(Coord {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        None
    }

    pub(crate) fn flags_and_coord_compatible(&self, code: u32) -> (Flags, Option<Coord>) {
        // Preserve original C behavior: flags are set when a layer is tested,
        // not only when the character is actually found in that layer.
        let mut flags = Flags {
            is_basic: true,
            is_shift: false,
            is_altgr: false,
        };

        if let Some(coord) = self.find_coord_in(Layer::Basic, code) {
            return (flags, Some(coord));
        }

        flags.is_shift = true;

        if let Some(coord) = self.find_coord_in(Layer::Shift, code) {
            return (flags, Some(coord));
        }

        flags.is_altgr = true;

        if let Some(coord) = self.find_coord_in(Layer::AltGr, code) {
            return (flags, Some(coord));
        }

        (flags, None)
    }

    pub(crate) fn collect_codes(&self, out: &mut Vec<char>, seen: &mut [bool]) {
        for map in [&self.basic, &self.shift, &self.altgr] {
            for &code in map.iter() {
                if code == INVALID_CODE {
                    continue;
                }

                if let Some(ch) = char::from_u32(code) {
                    let idx = code as usize;
                    if idx < seen.len() && !seen[idx] {
                        seen[idx] = true;
                        out.push(ch);
                    }
                }
            }
        }
    }
}
