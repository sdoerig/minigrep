extern crate getopts;
use getopts::Options;
use std::env;

use crate::store::config::Config;
use std::process;

pub fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} -f FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn get_config() -> Config {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mandatory_params = vec!["f", "p"];

    let mut case_sensitive = true;
    let mut opts = Options::new();
    opts.optopt("p", "pattern", "set pattern to find", "PATTERN");
    opts.optopt(
        "s",
        "substitute",
        "subsitute pattern with this",
        "SUBSTITUTE",
    );
    opts.optopt("f", "file", "file to search pattern in", "FILE");

    opts.optflag(
        "i",
        "insensitive",
        "case insensitive matching - not valued in case of a regex pattern",
    );
    opts.optflag("e", "regex", "interpret pattern as regular expression");
    opts.optflag("n", "number", "show line numbers of matches");
    opts.optflag("r", "recursiv", "search FILE recursiv");
    opts.optopt(
        "a",
        "from",
        "start matching at line number",
        "START_AT_LINE_NUMBER",
    );
    opts.optopt(
        "z",
        "until",
        "match as long as line number is smaller",
        "END_AT_LINE_NUMBER",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => {
            print_usage(&program, &opts);
            process::exit(2)
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        process::exit(10)
    }

    for mandatory_param in mandatory_params {
        if !matches.opt_present(mandatory_param) {
            print_usage(&program, &opts);
            process::exit(2)
        }
    }

    let pattern = match matches.opt_get("p") {
        Ok(m) => m.unwrap(),
        Err(_f) => {
            print_usage(&program, &opts);
            process::exit(2)
        }
    };

    let file = match matches.opt_get("f") {
        Ok(m) => m.unwrap(),
        Err(_f) => {
            print_usage(&program, &opts);
            process::exit(3)
        }
    };

    if matches.opt_present("i") {
        case_sensitive = false;
    }
    let mut config: Config = Config::new(pattern, file, case_sensitive).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if matches.opt_present("n") {
        config.set_show_line_number(true);
    }

    if matches.opt_present("e") {
        config.set_is_regex(true);
    }

    if matches.opt_present("r") {
        config.set_recursive(true);
    }

    if matches.opt_present("a") {
        config.set_start_matching_at(match matches.opt_get("a") {
            Ok(m) => m.unwrap(),
            Err(_f) => 0,
        });
    }

    if matches.opt_present("z") {
        config.set_end_matching_after(match matches.opt_get("z") {
            Ok(m) => m.unwrap(),
            Err(_f) => 0,
        });
    }

    if matches.opt_present("s") {
        config.set_is_substitute(true);
        config.set_substitute(match matches.opt_get("s") {
            Ok(m) => m.unwrap(),
            Err(_f) => String::from(""),
        });
    };

    config
}
