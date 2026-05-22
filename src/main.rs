mod app;
mod cli;
mod cli_short;
mod config;
mod constants;
mod error;
mod generator;
mod keymap;
mod model;
mod output;
mod parser;
mod route;

use std::process;

#[cfg(test)]
mod tests;

fn main() {
    match app::run() {
        Ok(code) => process::exit(code),
        Err(err) => {
            eprintln!("{err}");
            process::exit(255);
        }
    }
}
