extern crate num_cpus;
extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::config::config::Config as Config;
extern crate glob;
use glob::glob;




pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    type Output = fn(&MatchedLine);
    let mut output_ptr: Output = print_without_line_number;
    if config.show_line_number {
        output_ptr = print_with_line_number;        
    } else {

    }
    if config.recursiv {
        let pool = ThreadPool::new(num_cpus::get());

        let (tx, rx) = channel::<MatchedLine>();
        let glob_pattern = format!("./**/{}", config.filename);
        for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
            let path = entry.unwrap();
            let filename = path.into_os_string().into_string().unwrap();
            let tx_tmp = tx.clone();
            let config_tmp = config.clone();
            pool.execute(move || {
                let _open_file_success = match open_file(&filename, &config_tmp, tx_tmp) {
                    Ok(_ok) => true,
                    Err(_err) => false,
                };
                //tx.send(digest).expect("Could not send data!");
            })
            
            
        }
        for t in rx.iter() {
            let matched = t;
            output_ptr(&matched);
        }
        Ok(())
    } else {
        //open_file(&config.filename, &config)
        Ok(())
    }
}

fn open_file(filename: &String, config: &Config, tx: std::sync::mpsc::Sender<MatchedLine>) -> Result<(), Box<dyn Error>>  {
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    type Search = fn(&String, &Config, usize, String) -> MatchedLine;
    
    let mut search_ptr: Search = search;
    
    
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
        let line = line?;
        let matched = search_ptr(&filename, &config, _index + 1, line);
        tx.send(matched).expect("Could not send data!");
        //output_ptr(&matched);
    }
    Ok(())
}

struct MatchedLine {
    matched: bool,
    linenumber: usize,
    line: String,
    filename: String
}

fn print_with_line_number(matched: &MatchedLine) {
    if matched.matched {
        println!("{} {}: {}", matched.filename, matched.linenumber, matched.line);
    }
}

fn print_without_line_number(matched: &MatchedLine) {
    if matched.matched {
        println!("{}: {}", matched.filename, matched.line);
    }
}

fn search_regex_by_line(filename: &String, config: &Config, linenumber: usize, line: String) -> MatchedLine {
    MatchedLine{filename: filename.clone(), matched: config.regex.is_match(&line), linenumber: linenumber, line: line}
}

fn replace_regex_by_line(filename: &String, config: &Config, linenumber: usize, line: String) -> MatchedLine {
    if config.regex.is_match(&line) {
        let line_modified = config.regex.replace_all(&line, config.substitute.as_str());
        MatchedLine{filename: filename.clone(), matched: true, linenumber: linenumber, line: line_modified.to_owned().to_string()}
    } else {
        MatchedLine{filename: filename.clone(), matched: false, linenumber: linenumber, line: line}
    }
}

fn search(filename: &String, config: &Config, linenumber: usize, line: String) -> MatchedLine {
    MatchedLine{filename: filename.clone(), matched: line.contains(&config.query), linenumber: linenumber, line: line}
}

fn search_case_insensitive(filename: &String, config: &Config, linenumber: usize, line: String) -> MatchedLine {
    MatchedLine{filename: filename.clone(), matched: line.to_lowercase().contains(&config.query), linenumber: linenumber, line: line}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = String::from("safe, fast, productive.");
        let config = Config::new(String::from("duct"), 
            String::from("mey"), true, false, false, 
            String::from(""), false, false).unwrap();
        assert_eq!(
            true,
            search(&String::from("filename"), &config, 1, contents).matched
        );
    }

    #[test]
    fn case_insensitive() {
        let contents = String::from("Rust:");
        let mut config = Config::new(String::from("rUsT"), 
            String::from("mey"), false, false, false, 
            String::from(""), false, false).unwrap();
        config.query = String::from("rUsT");
        config.set_case_sensitive(false);
        assert_eq!(
            true,
            search_case_insensitive(&String::from("filename"), &config, 1, contents).matched
        );
    }

    

    #[test]
    fn case_regex_by_line() {
        let config = Config::new(String::from("e{2}"), String::from("mey"), true, false, false, 
            String::from("e{2}"), false, false).unwrap();
        let line = String::from("Pick three.");
        let matched_line = search_regex_by_line(&String::from("filename"), &config, 1, line);

        assert_eq!(true, matched_line.matched);
    }

    #[test]
    fn regex_example() {
        let config = Config::new(String::from("(?P<y>\\d{4})-(?P<m>\\d{2})-(?P<d>\\d{2})"), 
            String::from("mey"), true, false, false, String::from("$m/$d/$y"), false, false).unwrap();
        let line = String::from("2012-03-14, 2013-01-01 and 2014-07-05");
        let after = replace_regex_by_line(&String::from("filename"), &config, 1, line);
        assert_eq!(after.line, "03/14/2012, 01/01/2013 and 07/05/2014");
        println!("{}", after.line);
    }

}