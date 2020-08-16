use regex::Regex;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub is_regex: bool,
    pub is_subsitute: bool,
    pub substitute: String,
    pub regex: Regex,
    pub show_line_number: bool,
    pub recursiv: bool,
    pub start_matching_at: usize,
    pub end_matching_after: usize,
}

impl Config {
    pub fn new(
        query_tmp: String,
        filename: String,
        case_sensitive: bool,
    ) -> Result<Config, &'static str> {
        let regex = Regex::new(&query_tmp).unwrap();
        let query = match case_sensitive {
            true => query_tmp,
            false => query_tmp.to_lowercase(),
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
            is_regex: false,
            is_subsitute: false,
            substitute: String::from(""),
            regex,
            show_line_number: false,
            recursiv: false,
            start_matching_at: 0,
            end_matching_after: 0,
        })
    }

    pub fn set_is_regex(&mut self, is_regex: bool) {
        self.is_regex = is_regex;
    }

    pub fn set_is_substitute(&mut self, is_subsitute: bool) {
        self.is_subsitute = is_subsitute;
    }

    pub fn set_substitute(&mut self, substitute: String) {
        self.substitute = substitute;
    }

    pub fn set_show_line_number(&mut self, show_line_number: bool) {
        self.show_line_number = show_line_number;
    }

    pub fn set_recursive(&mut self, recursive: bool) {
        self.recursiv = recursive
    }

    pub fn set_start_matching_at(&mut self, start_matching_at: usize) {
        self.start_matching_at = start_matching_at;
    }

    pub fn set_end_matching_after(&mut self, end_matching_after: usize) {
        self.end_matching_after = end_matching_after
    }

    pub fn do_match(&self, line_counter: &usize) -> bool {
        (self.start_matching_at == 0 && self.end_matching_after == 0)
            || (self.start_matching_at <= *line_counter && *line_counter < self.end_matching_after)
            || (self.start_matching_at > 0
                && self.end_matching_after == 0
                && *line_counter >= self.start_matching_at)
            || (self.start_matching_at == 0
                && self.end_matching_after > 0
                && *line_counter < self.end_matching_after)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_insensitive_config() {
        let mut config = Config::new(String::from("TeST"), String::from("file"), false).unwrap();
        config.set_is_regex(false);
        config.set_is_substitute(false);
        config.set_substitute(String::from(""));
        config.set_show_line_number(false);
        config.set_recursive(true);
        config.set_start_matching_at(0);
        config.set_end_matching_after(0);
        assert_eq!(config.query, "test");
    }

    #[test]
    fn test_case_sensitive_config() {
        let mut config = Config::new(String::from("TeST"), String::from("file"), true).unwrap();

        config.set_is_regex(false);
        config.set_is_substitute(false);
        config.set_substitute(String::from(""));
        config.set_show_line_number(false);
        config.set_recursive(false);
        config.set_start_matching_at(1234567891234);
        config.set_end_matching_after(1234567890123456780);
        assert_eq!(config.query, "TeST");
    }

    #[test]
    fn test_case_do_match_start_and_end_set() {
        let mut config = Config::new(String::from("TeST"), String::from("file"), true).unwrap();
        config.set_is_regex(false);
        config.set_is_substitute(false);
        config.set_substitute(String::from(""));
        config.set_show_line_number(false);
        config.set_recursive(false);
        config.set_start_matching_at(3);
        config.set_end_matching_after(7);
        // Expected results where as check_cases[0] stands for line one an so on
        let check_cases = vec![false, false, true, true, true, true, false, false];
        for (line_counter, result) in check_cases.into_iter().enumerate() {
            //println!("{}", line_counter);
            assert_eq!(config.do_match(&(line_counter + 1)), result);
        }
    }
    #[test]
    fn test_case_do_match_start_set() {
        let mut config = Config::new(String::from("TeST"), String::from("file"), true).unwrap();
        config.set_is_regex(false);
        config.set_is_substitute(false);
        config.set_substitute(String::from(""));
        config.set_show_line_number(false);
        config.set_recursive(false);
        config.set_start_matching_at(3);
        config.set_end_matching_after(0);
        // Expected results where as check_cases[0] stands for line one an so on
        let check_cases = vec![false, false, true, true, true, true, true, true];
        for (line_counter, result) in check_cases.into_iter().enumerate() {
            //println!("{}", line_counter);
            assert_eq!(config.do_match(&(line_counter + 1)), result);
        }
    }

    #[test]
    fn test_case_do_match_start_greater_end() {
        let mut config = Config::new(String::from("TeST"), String::from("file"), true).unwrap();
        config.set_is_regex(false);
        config.set_is_substitute(false);
        config.set_substitute(String::from(""));
        config.set_show_line_number(false);
        config.set_recursive(false);
        config.set_start_matching_at(4);
        config.set_end_matching_after(3);
        // Expected results where as check_cases[0] stands for line one an so on
        let check_cases = vec![false, false, false, false, false, false, false, false];
        for (line_counter, result) in check_cases.into_iter().enumerate() {
            //println!("{}", line_counter);
            assert_eq!(config.do_match(&(line_counter + 1)), result);
        }
    }

    #[test]
    fn test_case_do_match_end_set() {
        let mut config = Config::new(String::from("TeST"), String::from("file"), true).unwrap();
        config.set_is_regex(false);
        config.set_is_substitute(false);
        config.set_substitute(String::from(""));
        config.set_show_line_number(false);
        config.set_recursive(false);
        config.set_start_matching_at(0);
        config.set_end_matching_after(3);
        // Expected results where as check_cases[0] stands for line one an so on
        let check_cases = vec![true, true, false, false, false, false, false, false];
        for (line_counter, result) in check_cases.into_iter().enumerate() {
            //println!("{}", line_counter);
            assert_eq!(config.do_match(&(line_counter + 1)), result);
        }
    }
}
