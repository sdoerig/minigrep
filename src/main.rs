use std::process;
extern crate getopts;
mod search;
mod store;
mod structs;
mod utility;
use search::minigrep::run;
use utility::parse_args::get_config;

fn main() {
    if let Err(e) = run(get_config()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
