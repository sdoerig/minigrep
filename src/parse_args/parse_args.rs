extern crate getopts;
use getopts::Options;
use std::env;

use crate::config::config::Config as Config;
use std::process;

pub fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} -f FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn get_config() -> Config {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mandatory_params = vec!["f", "p"];
    let (mut case_sensitive, 
        mut regex, 
        mut show_line_number, 
        mut do_substitution,
        mut recursiv) = (true, 
            false, 
            false, 
            false,
            false);
    let mut opts = Options::new();
    opts.optopt("p", "pattern", "set pattern to find", "PATTERN");
    opts.optopt("s", "substitute", "subsitute pattern with this", "SUBSTITUTE");
    opts.optopt("f", "file", "file to search pattern in", "FILE");

    opts.optflag("i", 
        "insensitive", 
        "case insensitive matching - not valued in case of a regex pattern");
    opts.optflag("e", "regex", "interpret pattern as regular expression");
    opts.optflag("n", 
        "number", 
        "show line numbers of matches");
    opts.optflag("r", 
        "recursiv", 
        "search FILE recursiv");
    opts.optopt("a", 
        "from", 
        "start matching at line number", "START_AT_LINE_NUMBER");
    opts.optopt("z", "until", "match as long as line number is smaller", "END_AT_LINE_NUMBER");
    opts.optflag("h", "help", "print this help menu");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_f) => { print_usage(&program, &opts);
            process::exit(2) }
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

    if matches.opt_present("i") {
        case_sensitive = false;
    }

    if matches.opt_present("n") {
        show_line_number = true;
    } 

    if matches.opt_present("e") {
        regex = true;
    }

    if matches.opt_present("r") {
        recursiv = true;
    }
    
    let pattern = match matches.opt_get("p") {
        Ok(m) => { m.unwrap() }
        Err(_f) => {print_usage(&program, &opts);
            process::exit(2)}
        
    };
    
    let file = match matches.opt_get("f") {
        Ok(m) => { m.unwrap() }
        Err(_f) => {print_usage(&program, &opts);
            process::exit(3)}
        
    };

    let mut start_at: usize = 0;
    if matches.opt_present("a") {
        start_at = match matches.opt_get("a") {
            Ok(m) => { m.unwrap() }
            Err(_f) => {0}
        };
    }

    let mut end_at: usize = 0;
    if matches.opt_present("z") {
        end_at = match matches.opt_get("z") {
            Ok(m) => { m.unwrap() }
            Err(_f) => {0}
        };
    }


    let mut subsitute = String::from("");
    if matches.opt_present("s") {
        subsitute = match matches.opt_get("s") {
            Ok(m) => { do_substitution = true;
                m.unwrap() }
            Err(_f) => {do_substitution = false;
            String::from("")}
        }
        
    };

    Config::new(pattern, file, 
        case_sensitive, regex, 
        do_substitution, subsitute, 
        show_line_number, recursiv,
        start_at, end_at).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    })
}
