use std::path::PathBuf;

use crate::cli::{c_atoi, parse_usize_atoi, Mode};
use crate::config::Config;
use crate::error::{KwpError, Result};

pub(crate) fn parse_short_option(
    raw: &[String],
    i: &mut usize,
    config: &mut Config,
    mode: &mut Mode,
) -> Result<()> {
    let arg = &raw[*i];
    let bytes = arg.as_bytes();

    if bytes.len() < 2 || bytes[0] != b'-' {
        return Ok(());
    }

    let opt = bytes[1] as char;
    let attached = if bytes.len() > 2 {
        Some(String::from_utf8_lossy(&bytes[2..]).into_owned())
    } else {
        None
    };

    let mut required_value = |name: &str| -> Result<String> {
        if let Some(value) = attached.clone() {
            return Ok(value);
        }

        let next = *i + 1;
        if next >= raw.len() {
            return Err(KwpError::Message(format!("Missing argument for -{name}")));
        }

        *i = next;
        Ok(raw[next].clone())
    };

    match opt {
        'V' => *mode = Mode::Version,
        'h' => *mode = Mode::Usage,
        'z' => config.user_mod_all = true,
        'c' => config.user_dir_cont = true,
        '0' => config.user_dir_all = true,
        'P' => config.compat_order = false,
        'C' => config.compat_order = true,
        'o' => config.output_file = Some(PathBuf::from(required_value("o")?)),
        'b' => config.user_mod_basic = c_atoi(&required_value("b")?),
        'B' => config.user_mod_basic = c_atoi(&required_value("B")?),
        's' => config.user_mod_shift = c_atoi(&required_value("s")?),
        'S' => config.user_mod_shift = c_atoi(&required_value("S")?),
        'a' => config.user_mod_altgr = c_atoi(&required_value("a")?),
        'A' => config.user_mod_altgr = c_atoi(&required_value("A")?),
        '1' => config.user_dir_south_west = c_atoi(&required_value("1")?),
        '2' => config.user_dir_south = c_atoi(&required_value("2")?),
        '3' => config.user_dir_south_east = c_atoi(&required_value("3")?),
        '4' => config.user_dir_west = c_atoi(&required_value("4")?),
        '5' => config.user_dir_repeat = c_atoi(&required_value("5")?),
        '6' => config.user_dir_east = c_atoi(&required_value("6")?),
        '7' => config.user_dir_north_west = c_atoi(&required_value("7")?),
        '8' => config.user_dir_north = c_atoi(&required_value("8")?),
        '9' => config.user_dir_north_east = c_atoi(&required_value("9")?),
        'n' => config.user_dist_min = parse_usize_atoi(&required_value("n")?),
        'x' => config.user_dist_max = parse_usize_atoi(&required_value("x")?),
        _ => return Err(KwpError::Message(format!("Unknown option -{opt}"))),
    }

    Ok(())
}
