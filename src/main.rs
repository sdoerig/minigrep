use std::env;

use std::process;
use minigrep;
use minigrep::Config;
extern crate getopts;
use getopts::Options;
mod parse_args;
use parse_args::print_usage as print_usage;
use parse_args::get_config as get_config;


fn main() {


    if let Err(e) = minigrep::run(get_config()) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
