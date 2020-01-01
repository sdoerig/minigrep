use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::config::config::Config as Config;




pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.filename).unwrap();
    let reader = BufReader::new(file);
    type Search = fn(&Config, String) -> MatchedLine;
    type Output = fn(&MatchedLine, usize);
    let mut search_ptr: Search = search;
    let mut output_ptr: Output = print_without_line_number;
    if config.show_line_number {
        output_ptr = print_with_line_number;        
    } else {

    }
    if config.is_regex {
        if config.is_subsitute {
            search_ptr = replace_regex_by_line;
        } else {
            search_ptr = search_regex_by_line;
        }
    } else if config.case_sensitive == false {
        search_ptr = search_case_insensitive;
    }
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let matched = search_ptr(&config, line);
        output_ptr(&matched, _index);
    }
    
    Ok(())
}




struct MatchedLine {
    matched: bool,
    line: String
}

fn print_with_line_number(matched: &MatchedLine, line_number: usize) {
    if matched.matched {
        println!("{}: {}", line_number + 1, matched.line);
    }
}

fn print_without_line_number(matched: &MatchedLine, line_number: usize) {
    if matched.matched {
        println!("{}", matched.line);
    }
}

fn search_regex_by_line(config: &Config, line: String) -> MatchedLine {
    MatchedLine{matched: config.regex.is_match(&line), line: line}
}

fn replace_regex_by_line(config: &Config, line: String) -> MatchedLine {
    if config.regex.is_match(&line) {
        let line_modified = config.regex.replace_all(&line, config.substitute.as_str());
        MatchedLine{matched: true, line: line_modified.to_owned().to_string()}
    } else {
        MatchedLine{matched: false, line: line}
    }
}



fn search(config: &Config, line: String) -> MatchedLine {
    MatchedLine{matched: line.contains(&config.query), line: line}
}

fn search_case_insensitive(config: &Config, line: String) -> MatchedLine {
    MatchedLine{matched: line.to_lowercase().contains(&config.query), line: line}


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        
        let contents = String::from("safe, fast, productive.");
        let config = Config::new(String::from("duct"), 
            String::from("mey"), true, false, false, 
            String::from(""), false).unwrap();
        assert_eq!(
            true,
            search(&config, contents).matched
        );
    }

    #[test]
    fn case_insensitive() {
        let contents = String::from("Rust:");
        let mut config = Config::new(String::from("rUsT"), 
            String::from("mey"), false, false, false, 
            String::from(""), false).unwrap();
        config.query = String::from("rUsT");
        config.set_case_sensitive(false);
        assert_eq!(
            true,
            search_case_insensitive(&config, contents).matched
        );
    }

    

    #[test]
    fn case_regex_by_line() {
        let config = Config::new(String::from("e{2}"), String::from("mey"), true, false, false, 
            String::from("e{2}"), false).unwrap();
        let line = String::from("Pick three.");
        let matched_line = search_regex_by_line(&config, line);

        assert_eq!(true, matched_line.matched);
    }

    #[test]
    fn regex_example() {
        let config = Config::new(String::from("(?P<y>\\d{4})-(?P<m>\\d{2})-(?P<d>\\d{2})"), 
            String::from("mey"), true, false, false, String::from("$m/$d/$y"), false).unwrap();
        let line = String::from("2012-03-14, 2013-01-01 and 2014-07-05");
        let after = replace_regex_by_line(&config, line);
        assert_eq!(after.line, "03/14/2012, 01/01/2013 and 07/05/2014");
        println!("{}", after.line);
    }

}