use std::path::PathBuf;

use crate::cli_short::parse_short_option;
use crate::config::Config;
use crate::error::{KwpError, Result};

pub(crate) struct ParsedArgs {
    pub(crate) mode: Mode,
    pub(crate) config: Config,
    pub(crate) basechars_file: PathBuf,
    pub(crate) keymap_file: PathBuf,
    pub(crate) routes_file: PathBuf,
}

pub(crate) enum Mode {
    Run,
    Usage,
    Version,
}

pub(crate) fn usage_mini(prog: &str) -> String {
    format!("Usage: {prog} [options]... basechars-file keymap-file routes-file\n\nTry --help for more help.\n")
}

pub(crate) fn usage_big(prog: &str) -> String {
    format!(
        "Advanced keyboard-walk generator with configurable basechars, keymap and routes\n\n\
Usage: {prog} [options]... basechars-file keymap-file routes-file\n\n\
 Options Short / Long        | Type | Description                                                 | Default\n\
=============================+======+=============================================================+=========\n\
  -V, --version              |      | Print version                                               |\n\
  -h, --help                 |      | Print help                                                  |\n\
  -o, --output-file          | FILE | Output-file                                                 |\n\
  -b, --keyboard-basic       | BOOL | Include characters reachable without holding shift or altgr | 1\n\
  -s, --keyboard-shift       | BOOL | Include characters reachable by holding shift               | 0\n\
  -a, --keyboard-altgr       | BOOL | Include characters reachable by holding altgr (non-english) | 0\n\
  -z, --keyboard-all         |      | Shortcut to enable all --keyboard-* modifier                |\n\
  -1, --keywalk-south-west   | BOOL | Include routes heading diagonal south-west                  | 0\n\
  -2, --keywalk-south        | BOOL | Include routes heading straight south                       | 1\n\
  -3, --keywalk-south-east   | BOOL | Include routes heading diagonal south-east                  | 0\n\
  -4, --keywalk-west         | BOOL | Include routes heading straight west                        | 1\n\
  -5, --keywalk-repeat       | BOOL | Include routes repeating character                          | 0\n\
  -6, --keywalk-east         | BOOL | Include routes heading straight east                        | 1\n\
  -7, --keywalk-north-west   | BOOL | Include routes heading diagonal north-west                  | 0\n\
  -8, --keywalk-north        | BOOL | Include routes heading straight north                       | 1\n\
  -9, --keywalk-north-east   | BOOL | Include routes heading diagonal north-east                  | 0\n\
  -c, --keywalk-cont         |      | Shortcut to enable adjacent keys (continuous walks)         |\n\
  -0, --keywalk-all          |      | Shortcut to enable all --keywalk-* directions               |\n\
  -n, --keywalk-distance-min | NUM  | Minimum allowed distance between keys                       | 1\n\
  -x, --keywalk-distance-max | NUM  | Maximum allowed distance between keys                       | 1\n\
  -P, --fast-practical       |      | Use fast practical generation order                         | default\n\
  -C, --compat-order         |      | Preserve original C output order; slower on sparse walks    |\n"
    )
}

pub(crate) fn parse_args(raw: Vec<String>) -> Result<ParsedArgs> {
    let prog = raw.first().map(String::as_str).unwrap_or("kwp-rs");
    let mut config = Config::default();
    let mut mode = Mode::Run;
    let mut positionals = Vec::with_capacity(3);
    let mut i = 1;

    while i < raw.len() {
        let arg = &raw[i];

        if arg == "--" {
            positionals.extend(raw[i + 1..].iter().cloned());
            break;
        }

        if !arg.starts_with('-') || arg == "-" {
            positionals.push(arg.clone());
            i += 1;
            continue;
        }

        if let Some(long) = arg.strip_prefix("--") {
            let (name, inline_value) = match long.split_once('=') {
                Some((name, value)) => (name, Some(value.to_string())),
                None => (long, None),
            };

            match name {
                "version" => mode = Mode::Version,
                "help" => mode = Mode::Usage,
                "keyboard-all" => config.user_mod_all = true,
                "keywalk-cont" => config.user_dir_cont = true,
                "keywalk-all" => config.user_dir_all = true,
                "fast-practical" => config.compat_order = false,
                "compat-order" => config.compat_order = true,
                "output-file" => {
                    config.output_file = Some(PathBuf::from(take_value(
                        &raw,
                        &mut i,
                        inline_value,
                        "--output-file",
                    )?));
                }
                "keyboard-basic" => {
                    config.user_mod_basic =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keyboard-basic")?)
                }
                "keyboard-shift" => {
                    config.user_mod_shift =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keyboard-shift")?)
                }
                "keyboard-altgr" => {
                    config.user_mod_altgr =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keyboard-altgr")?)
                }
                "keywalk-south-west" => {
                    config.user_dir_south_west = c_atoi(&take_value(
                        &raw,
                        &mut i,
                        inline_value,
                        "--keywalk-south-west",
                    )?)
                }
                "keywalk-south" => {
                    config.user_dir_south =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keywalk-south")?)
                }
                "keywalk-south-east" => {
                    config.user_dir_south_east = c_atoi(&take_value(
                        &raw,
                        &mut i,
                        inline_value,
                        "--keywalk-south-east",
                    )?)
                }
                "keywalk-west" => {
                    config.user_dir_west =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keywalk-west")?)
                }
                "keywalk-repeat" => {
                    config.user_dir_repeat =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keywalk-repeat")?)
                }
                "keywalk-east" => {
                    config.user_dir_east =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keywalk-east")?)
                }
                "keywalk-north-west" => {
                    config.user_dir_north_west = c_atoi(&take_value(
                        &raw,
                        &mut i,
                        inline_value,
                        "--keywalk-north-west",
                    )?)
                }
                "keywalk-north" => {
                    config.user_dir_north =
                        c_atoi(&take_value(&raw, &mut i, inline_value, "--keywalk-north")?)
                }
                "keywalk-north-east" => {
                    config.user_dir_north_east = c_atoi(&take_value(
                        &raw,
                        &mut i,
                        inline_value,
                        "--keywalk-north-east",
                    )?)
                }
                "keywalk-distance-min" => {
                    config.user_dist_min = parse_usize_atoi(&take_value(
                        &raw,
                        &mut i,
                        inline_value,
                        "--keywalk-distance-min",
                    )?)
                }
                "keywalk-distance-max" => {
                    config.user_dist_max = parse_usize_atoi(&take_value(
                        &raw,
                        &mut i,
                        inline_value,
                        "--keywalk-distance-max",
                    )?)
                }
                _ => {
                    return Err(KwpError::Message(format!(
                        "Unknown option --{name}\n{}",
                        usage_mini(prog)
                    )))
                }
            }

            i += 1;
            continue;
        }

        parse_short_option(&raw, &mut i, &mut config, &mut mode)?;
        i += 1;
    }

    config.apply_shortcuts_and_validate()?;

    if matches!(mode, Mode::Usage | Mode::Version) {
        return Ok(ParsedArgs {
            mode,
            config,
            basechars_file: PathBuf::new(),
            keymap_file: PathBuf::new(),
            routes_file: PathBuf::new(),
        });
    }

    if positionals.len() != 3 {
        return Err(KwpError::Message(usage_mini(prog)));
    }

    Ok(ParsedArgs {
        mode,
        config,
        basechars_file: PathBuf::from(&positionals[0]),
        keymap_file: PathBuf::from(&positionals[1]),
        routes_file: PathBuf::from(&positionals[2]),
    })
}

pub(crate) fn take_value(
    raw: &[String],
    i: &mut usize,
    inline_value: Option<String>,
    option_name: &str,
) -> Result<String> {
    if let Some(value) = inline_value {
        return Ok(value);
    }

    let next = *i + 1;
    if next >= raw.len() {
        return Err(KwpError::Message(format!(
            "Missing argument for {option_name}"
        )));
    }

    *i = next;
    Ok(raw[next].clone())
}

pub(crate) fn c_atoi(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    let mut sign = 1i32;
    if i < bytes.len() {
        if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        } else if bytes[i] == b'+' {
            i += 1;
        }
    }

    let mut value = 0i32;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        value = value
            .saturating_mul(10)
            .saturating_add((bytes[i] - b'0') as i32);
        i += 1;
    }

    value.saturating_mul(sign)
}

pub(crate) fn parse_usize_atoi(s: &str) -> usize {
    let value = c_atoi(s);
    if value <= 0 {
        0
    } else {
        value as usize
    }
}
