use std::error::Error;
use std::fs::File;
use std::fmt;
use std::io::{BufRead, BufReader};
use crate::config::config::Config as Config;
extern crate glob;
use glob::glob;




pub fn run( config: Config) -> Result<(), Box<dyn Error>> {
    
    if config.recursiv {
        let glob_pattern = format!("./**/{}", config.filename);
        for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
            let path = entry.unwrap();
            let filename = path.into_os_string().into_string().unwrap();
            let _open_file_success = match open_file(&filename, &config) {
                Ok(_ok) => true,
                Err(_err) => false,
            };
            
        }
        Ok(())
    } else {
        open_file(&config.filename, &config)
    }
}

fn open_file(filename: &str, config: &Config) -> Result<(), Box<dyn Error>>  {
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    type Search = fn(&Config, String) -> MatchedLine;
    type Output = fn(&str, &MatchedLine, usize);
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
    } else if !config.case_sensitive {
        search_ptr = search_case_insensitive;
    }
    if config.start_matching_at > 0 || config.end_matching_after > 0 {
        for (_index, line) in reader.lines().enumerate() {
            if config.do_match(&_index) {
                let line = line?;
                let matched = search_ptr(&config, line);
                let printable = PrintableWithFileNameLineNumber{filename: &filename, line_number: _index, matched};
                print_line(&printable);
            }
        }
    } else {
        for (_index, line) in reader.lines().enumerate() {
            let line = line?;
            let matched = search_ptr(&config, line);
            let printable = PrintableWithFileNameLineNumber{filename: &filename, line_number: _index, matched};
            print_line(&printable);
        }
    }
    
    Ok(())
}

struct MatchedLine {
    matched: bool,
    line: String
}

impl fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

struct PrintableWithFileNameLineNumber<'a>{
    matched: MatchedLine,
    filename: &'a str,
    line_number: usize
}
pub trait Matched: fmt::Display {
    fn matched(&self) -> bool;
}

impl Matched for PrintableWithFileNameLineNumber<'_> {
    fn matched(&self) -> bool {
        self.matched.matched    
    }
}

impl fmt::Display for PrintableWithFileNameLineNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}: {}", self.filename, self.line_number, self.matched)
    }
}

fn print_line(line: &dyn Matched) {
    if line.matched() {
        println!("&dyn Matched {}", format!("{}", line));
    }
}


fn print_with_line_number(filename: &str, matched: &MatchedLine, line_number: usize) {
    if matched.matched {
        println!("{} {}: {}", filename, line_number + 1, matched.line);
    }
}

fn print_without_line_number(filename: &str, matched: &MatchedLine, _line_number: usize) {
    if matched.matched {
        println!("{}: {}", filename, matched.line);
    }
}

fn search_regex_by_line(config: &Config, line: String) -> MatchedLine {
    MatchedLine{matched: config.regex.is_match(&line), line}
}

fn replace_regex_by_line(config: &Config, line: String) -> MatchedLine {
    if config.regex.is_match(&line) {
        let line_modified = config.regex.replace_all(&line, config.substitute.as_str());
        MatchedLine{matched: true, line: line_modified.to_owned().to_string()}
    } else {
        MatchedLine{matched: false, line}
    }
}

fn search(config: &Config, line: String) -> MatchedLine {
    MatchedLine{matched: line.contains(&config.query), line}
}

fn search_case_insensitive(config: &Config, line: String) -> MatchedLine {
    MatchedLine{matched: line.to_lowercase().contains(&config.query), line}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        
        let contents = String::from("safe, fast, productive.");
        let config = Config::new(String::from("duct"), 
            String::from("mey"), true, false, false, 
            String::from(""), false, false, 0,0).unwrap();
        assert_eq!(
            true,
            search(&config, contents).matched
        );
    }

    #[test]
    fn case_insensitive() {
        let contents = String::from("Rust:");
        let config = Config::new(String::from("rUsT"), 
            String::from("mey"), false, false, false, 
            String::from(""), false, false, 0, 0).unwrap();
        assert_eq!(
            true,
            search_case_insensitive(&config, contents).matched
        );
    }

    

    #[test]
    fn case_regex_by_line() {
        let config = Config::new(String::from("e{2}"), String::from("mey"), true, false, false, 
            String::from("e{2}"), false, false, 0, 0).unwrap();
        let line = String::from("Pick three.");
        let matched_line = search_regex_by_line(&config, line);

        assert_eq!(true, matched_line.matched);
    }

    #[test]
    fn regex_example() {
        let config = Config::new(String::from("(?P<y>\\d{4})-(?P<m>\\d{2})-(?P<d>\\d{2})"), 
            String::from("mey"), true, false, false, String::from("$m/$d/$y"), false, false, 0, 0).unwrap();
        let line = String::from("2012-03-14, 2013-01-01 and 2014-07-05");
        let after = replace_regex_by_line(&config, line);
        assert_eq!(after.line, "03/14/2012, 01/01/2013 and 07/05/2014");
        println!("{}", after.line);
    }

    #[test]
    fn printable_with_filename_and_linenumber() {
        let matched = MatchedLine{matched: true, line: String::from("testLine")};
        let pwfn = PrintableWithFileNameLineNumber{line_number: 123, filename: &String::from("fileName"), matched };
        assert_eq!("fileName: 123: testLine",
           format!("{}", pwfn));
        print_line(&pwfn);
    }

}

