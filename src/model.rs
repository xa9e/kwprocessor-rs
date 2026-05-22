use crate::config::Config;
use crate::constants::{
    DIR_CNT, DIST_CNT, INVALID_CODE, INVALID_IDX, KEYMAP_ENTRIES, MOD_CNT, UNICODE_LIMIT,
    UNSET_INDEX,
};
use crate::error::{KwpError, Result};
use crate::keymap::Keymaps;

#[derive(Clone, Copy)]
pub(crate) struct EncodedChar {
    pub(crate) bytes: [u8; 4],
    pub(crate) len: u8,
}

impl EncodedChar {
    pub(crate) fn from_char(ch: char) -> Self {
        let mut bytes = [0u8; 4];
        let len = ch.encode_utf8(&mut bytes).len() as u8;
        Self { bytes, len }
    }
}

pub(crate) struct CharState {
    pub(crate) next: Box<[usize]>,
}

pub(crate) struct Model {
    pub(crate) states: Vec<CharState>,
    pub(crate) encoded: Vec<EncodedChar>,
    pub(crate) index_by_code: Vec<usize>,
}

pub(crate) fn build_model(keymaps: &Keymaps, basechars: &[char], config: &Config) -> Result<Model> {
    let dist_count = config.dist_count();
    let mod_count = config.mod_count();
    let dir_count = config.dir_count();
    let selection_count = dist_count
        .checked_mul(mod_count)
        .and_then(|value| value.checked_mul(dir_count))
        .ok_or_else(|| KwpError::Message("Selection count overflow".to_string()))?;

    if selection_count > DIST_CNT * MOD_CNT * DIR_CNT {
        return Err(KwpError::Message(format!(
            "Selection count {selection_count} is outside supported layout capacity"
        )));
    }

    let mut symbols = Vec::with_capacity(KEYMAP_ENTRIES * 3 + basechars.len());
    let mut seen = vec![false; UNICODE_LIMIT];

    keymaps.collect_codes(&mut symbols, &mut seen);

    for &ch in basechars {
        let code = ch as usize;
        if code < seen.len() && !seen[code] {
            seen[code] = true;
            symbols.push(ch);
        }
    }

    let mut index_by_code = vec![UNSET_INDEX; UNICODE_LIMIT];
    let mut encoded = Vec::with_capacity(symbols.len());

    for (idx, &ch) in symbols.iter().enumerate() {
        index_by_code[ch as usize] = idx;
        encoded.push(EncodedChar::from_char(ch));
    }

    let modifier_slots = config.modifier_slots();
    let direction_slots = config.direction_slots();
    let mut states = Vec::with_capacity(symbols.len());

    for &ch in &symbols {
        let (_, coord) = keymaps.flags_and_coord_compatible(ch as u32);
        let mut next = vec![INVALID_IDX; selection_count].into_boxed_slice();

        if let Some(coord) = coord {
            for selection in 0..selection_count {
                let distance_idx = selection % dist_count;
                let m2 = selection / dist_count;
                let modifier_idx = m2 % mod_count;
                let direction_idx = (m2 / mod_count) % dir_count;

                let Some(Some(layer)) = modifier_slots.get(modifier_idx).copied() else {
                    continue;
                };
                let Some(Some((dx, dy))) = direction_slots.get(direction_idx).copied() else {
                    continue;
                };

                let distance = (config.user_dist_min + distance_idx) as i32;
                let code = keymaps.char_at(layer, coord.x + dx * distance, coord.y + dy * distance);

                if code == INVALID_CODE {
                    continue;
                }

                let idx = *index_by_code.get(code as usize).unwrap_or(&UNSET_INDEX);
                if idx != UNSET_INDEX {
                    next[selection] = idx;
                }
            }
        }

        states.push(CharState { next });
    }

    Ok(Model {
        states,
        encoded,
        index_by_code,
    })
}
