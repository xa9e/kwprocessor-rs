use std::cell::RefCell;
use std::io::{self, Write};
use std::path::PathBuf;
use std::rc::Rc;

use crate::cli::{c_atoi, parse_args};
use crate::config::Config;
use crate::generator::generate;
use crate::model::build_model;
use crate::output::Output;
use crate::parser::{parse_basechars_file, parse_keymap_file, parse_routes_file};

#[derive(Clone)]
struct SharedBuffer(Rc<RefCell<Vec<u8>>>);

impl Write for SharedBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn sample_output(mut config: Config) -> Vec<u8> {
    config.apply_shortcuts_and_validate().unwrap();

    let keymaps = parse_keymap_file(&PathBuf::from("keymaps/en-us.keymap")).unwrap();
    let basechars =
        parse_basechars_file(&PathBuf::from("basechars/tiny.base"), &keymaps, &config).unwrap();
    let routes =
        parse_routes_file(&PathBuf::from("routes/2-to-4-exhaustive-prince.route")).unwrap();
    let model = build_model(&keymaps, &basechars, &config).unwrap();
    let buffer = Rc::new(RefCell::new(Vec::new()));
    let writer = SharedBuffer(buffer.clone());
    let mut output = Output::new(Box::new(writer));

    generate(&config, &basechars, &routes, &model, &mut output).unwrap();

    let bytes = buffer.borrow().clone();
    bytes
}

fn sorted_lines(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut lines: Vec<Vec<u8>> = bytes
        .split(|&byte| byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(Vec::from)
        .collect();
    lines.sort();
    lines
}

#[test]
fn fast_practical_is_default_and_compat_is_opt_in() {
    let parsed = parse_args(vec![
        "kwp-rs".to_string(),
        "basechars/tiny.base".to_string(),
        "keymaps/en-us.keymap".to_string(),
        "routes/2-to-4-exhaustive-prince.route".to_string(),
    ])
    .unwrap();
    assert!(!parsed.config.compat_order);

    let parsed = parse_args(vec![
        "kwp-rs".to_string(),
        "--compat-order".to_string(),
        "basechars/tiny.base".to_string(),
        "keymaps/en-us.keymap".to_string(),
        "routes/2-to-4-exhaustive-prince.route".to_string(),
    ])
    .unwrap();
    assert!(parsed.config.compat_order);

    let parsed = parse_args(vec![
        "kwp-rs".to_string(),
        "--compat-order".to_string(),
        "--fast-practical".to_string(),
        "basechars/tiny.base".to_string(),
        "keymaps/en-us.keymap".to_string(),
        "routes/2-to-4-exhaustive-prince.route".to_string(),
    ])
    .unwrap();
    assert!(!parsed.config.compat_order);
}

#[test]
fn fast_generator_matches_compat_output_set_on_sample() {
    let compat = Config {
        compat_order: true,
        ..Config::default()
    };

    let fast_output = sample_output(Config::default());
    let compat_output = sample_output(compat);

    assert!(!fast_output.is_empty());
    assert_eq!(sorted_lines(&fast_output), sorted_lines(&compat_output));
}

#[test]
fn c_atoi_matches_original_style_prefix_parsing() {
    assert_eq!(c_atoi("  -12abc34"), -12);
    assert_eq!(c_atoi("+7x"), 7);
    assert_eq!(c_atoi("abc"), 0);
}
