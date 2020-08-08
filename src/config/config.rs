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
    pub end_matching_after: usize
}

impl Config {
    pub fn new(query_tmp: String, 
        filename: String, 
        case_sensitive: bool, 
        is_regex: bool, 
        is_subsitute: bool, 
        substitute: String,
        show_line_number: bool,
        recursiv: bool,
        start_matching_at: usize,
        end_matching_after: usize
    ) -> Result<Config, &'static str> {
            let regex = Regex::new(&query_tmp).unwrap();
            let query = match case_sensitive {
                true => {query_tmp},
                false => {query_tmp.to_lowercase()}
            };
            Ok(Config { query , 
                filename, 
                case_sensitive, 
                is_regex, 
                is_subsitute, 
                substitute, 
                regex, 
                show_line_number, 
                recursiv, 
                start_matching_at, 
                end_matching_after})
        }
    
    pub fn do_match(&self, line_counter: &usize) -> bool {
        if (self.start_matching_at == 0 && self.end_matching_after == 0) || 
           (self.start_matching_at > 0 && self.end_matching_after > 0 && 
            self.start_matching_at <= *line_counter && *line_counter <= self.end_matching_after) || 
            (self.start_matching_at > 0 && self.end_matching_after == 0 && *line_counter >= self.start_matching_at) || 
            (self.start_matching_at == 0 && self.end_matching_after > 0 && *line_counter <= self.end_matching_after) {
            true
        } else {
            false
        }
    }

    

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_insensitive_config() {
        let config = Config::new(String::from("TeST"), 
            String::from("file"), 
            false, false, false, 
            String::from(""), false, true, 0, 0).unwrap();
        assert_eq!(config.query, "test");
    }

    #[test]
    fn test_case_sensitive_config() {
        let config = Config::new(String::from("TeST"), 
            String::from("file"), 
            true, false, false, 
            String::from(""), false, false, 1234567891234, 1234567890123456780).unwrap();
        assert_eq!(config.query, "TeST");
    }

    #[test]
    fn test_case_do_match_start_and_end_set() {
        let config = Config::new(String::from("TeST"), 
            String::from("file"), 
            true, false, false, 
            String::from(""), false, false, 3, 7).unwrap();
        // would be line 1 
        let check_cases = vec![ false, false, true, true, true, true, true, false];
        //let mut line_counter: usize = 0;
        for (line_counter, result) in check_cases.into_iter().enumerate() {
            //println!("{}", line_counter);
            
            assert_eq!(config.do_match(&(line_counter + 1)), result);
        }
        // line 2
        //assert_eq!(config.do_match(), false);
        // line 3
        //assert_eq!(config.do_match(), true);
        // line 4
        //assert_eq!(config.do_match(), true);
        // line 5
        //assert_eq!(config.do_match(), true);
        // line 6
        //assert_eq!(config.do_match(), true);
        // line 7
        //assert_eq!(config.do_match(), true);
        // line 8
        //assert_eq!(config.do_match(), false);

    }
    
}

