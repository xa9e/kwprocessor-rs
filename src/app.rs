use std::env;

use crate::cli::{parse_args, usage_big, Mode};
use crate::constants::VERSION_BIN;
use crate::error::Result;
use crate::generator::generate;
use crate::model::build_model;
use crate::output::{open_output, Output};
use crate::parser::{parse_basechars_file, parse_keymap_file, parse_routes_file};

pub fn run() -> Result<i32> {
    let args: Vec<String> = env::args().collect();
    let prog = args
        .first()
        .cloned()
        .unwrap_or_else(|| "kwp-rs".to_string());
    let parsed = parse_args(args)?;

    match parsed.mode {
        Mode::Usage => {
            print!("{}", usage_big(&prog));
            return Ok(0);
        }
        Mode::Version => {
            println!("v{:4.02}", VERSION_BIN as f64 / 100.0);
            return Ok(0);
        }
        Mode::Run => {}
    }

    let keymaps = parse_keymap_file(&parsed.keymap_file)?;
    let basechars = parse_basechars_file(&parsed.basechars_file, &keymaps, &parsed.config)?;
    let routes = parse_routes_file(&parsed.routes_file)?;
    let model = build_model(&keymaps, &basechars, &parsed.config)?;
    let writer = open_output(&parsed.config)?;
    let mut output = Output::new(writer);

    generate(&parsed.config, &basechars, &routes, &model, &mut output)?;

    Ok(0)
}
