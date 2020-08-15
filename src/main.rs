use std::process;
extern crate getopts;
mod parse_args;
mod config;
mod minigrep;
mod structs;
use crate::minigrep::minigrep::run as run;
use parse_args::parse_args::get_config;


fn main() {
    if let Err(e) = run(get_config()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
