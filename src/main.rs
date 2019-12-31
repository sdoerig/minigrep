use std::process;
use minigrep;
extern crate getopts;
mod parse_args;
use parse_args::get_config as get_config;


fn main() {


    if let Err(e) = minigrep::run(get_config()) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
