use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::constants::{
    KEYMAP_HEIGHT, KEYMAP_WIDTH, ROUTE_LENGTH_MAX, ROUTE_LENGTH_MIN, ROUTE_REPEAT_MAX,
    ROUTE_REPEAT_MIN,
};
use crate::error::{KwpError, Result};
use crate::keymap::{Keymaps, Layer};
use crate::route::Route;

pub(crate) fn logical_lines_c(content: &str) -> Vec<&str> {
    if content.is_empty() {
        Vec::new()
    } else {
        content.split_inclusive('\n').collect()
    }
}

pub(crate) fn strip_line_ending(mut line: &str) -> &str {
    while line.ends_with('\n') || line.ends_with('\r') {
        line = &line[..line.len() - 1];
    }
    line
}

pub(crate) fn parse_keymap_file(path: &PathBuf) -> Result<Keymaps> {
    let content = fs::read_to_string(path)
        .map_err(|err| KwpError::Message(format!("{}: {err}", path.display())))?;
    let lines = logical_lines_c(&content);

    if lines.len() != 12 {
        return Err(KwpError::Message(
            "Invalid keymap, not exactly 12 lines".to_string(),
        ));
    }

    let mut keymaps = Keymaps::new();

    for (y, line) in lines.iter().take(KEYMAP_HEIGHT).enumerate() {
        parse_keymap_line(
            &mut keymaps,
            Layer::Basic,
            strip_line_ending(line),
            y,
            y + 1,
            "basic",
        )?;
    }

    for (y, line) in lines
        .iter()
        .skip(KEYMAP_HEIGHT)
        .take(KEYMAP_HEIGHT)
        .enumerate()
    {
        parse_keymap_line(
            &mut keymaps,
            Layer::Shift,
            strip_line_ending(line),
            y,
            y + 5,
            "shift",
        )?;
    }

    for (y, line) in lines
        .iter()
        .skip(KEYMAP_HEIGHT * 2)
        .take(KEYMAP_HEIGHT)
        .enumerate()
    {
        parse_keymap_line(
            &mut keymaps,
            Layer::AltGr,
            strip_line_ending(line),
            y,
            y + 9,
            "altgr",
        )?;
    }

    Ok(keymaps)
}

pub(crate) fn parse_keymap_line(
    keymaps: &mut Keymaps,
    layer: Layer,
    line: &str,
    y: usize,
    total_line_num: usize,
    section_name: &str,
) -> Result<()> {
    let line_len = line.chars().count();

    if line_len > KEYMAP_WIDTH {
        return Err(KwpError::Message(format!(
            "ERROR: Keymap file format error.\n       Line {total_line_num} ({section_name} map, row {}) is too long.\n       Maximum allowed width is {KEYMAP_WIDTH} characters, but this line has {line_len}.",
            y + 1
        )));
    }

    for (x, ch) in line.chars().enumerate() {
        if ch == ' ' {
            continue;
        }

        keymaps.set(layer, x, y, ch as u32);
    }

    Ok(())
}

pub(crate) fn parse_basechars_file(
    path: &PathBuf,
    keymaps: &Keymaps,
    config: &Config,
) -> Result<Vec<char>> {
    let content = fs::read_to_string(path)
        .map_err(|err| KwpError::Message(format!("{}: {err}", path.display())))?;
    let lines = logical_lines_c(&content);

    if lines.len() != 1 {
        return Err(KwpError::Message(
            "Invalid basechars, not exactly 1 line".to_string(),
        ));
    }

    let line = strip_line_ending(lines[0]);
    let line_len = line.chars().count();

    if !(1..=1023).contains(&line_len) {
        return Err(KwpError::Message(format!(
            "{}: Invalid basechars",
            path.display()
        )));
    }

    let mut basechars = Vec::with_capacity(line_len);

    for ch in line.chars() {
        let (flags, _) = keymaps.flags_and_coord_compatible(ch as u32);

        if config.user_mod_basic == 0 && flags.is_basic {
            continue;
        }
        if config.user_mod_shift == 0 && flags.is_shift {
            continue;
        }
        if config.user_mod_altgr == 0 && flags.is_altgr {
            continue;
        }

        basechars.push(ch);
    }

    Ok(basechars)
}

pub(crate) fn parse_routes_file(path: &PathBuf) -> Result<Vec<Route>> {
    let content = fs::read_to_string(path)
        .map_err(|err| KwpError::Message(format!("{}: {err}", path.display())))?;
    let lines = logical_lines_c(&content);
    let mut routes = Vec::with_capacity(lines.len());

    for raw in lines {
        let line = strip_line_ending(raw);
        let line_len = line.chars().count();

        if !(ROUTE_LENGTH_MIN..=ROUTE_LENGTH_MAX).contains(&line_len) {
            continue;
        }

        let mut route = Route::new();

        for ch in line.chars() {
            let repeat = hex_convert(ch);

            if repeat < ROUTE_REPEAT_MIN as u32 || repeat > ROUTE_REPEAT_MAX as u32 {
                continue;
            }

            route.repeat[route.changes] = repeat as u8;
            route.changes += 1;
        }

        routes.push(route);
    }

    if routes.is_empty() {
        return Err(KwpError::Message(format!(
            "{}: no routes load",
            path.display()
        )));
    }

    Ok(routes)
}

pub(crate) fn hex_convert(ch: char) -> u32 {
    let c = ch as u32;
    (c & 15) + (c >> 6) * 9
}
