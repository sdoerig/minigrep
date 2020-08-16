use std::process;
extern crate getopts;
mod utility;
mod store;
mod search;
mod structs;
use search::minigrep::run as run;
use utility::parse_args::get_config;


fn main() {
    if let Err(e) = run(get_config()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
