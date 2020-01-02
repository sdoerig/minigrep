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
    pub recursiv: bool
}

impl Config {
    pub fn new(query_tmp: String, 
        filename: String, 
        case_sensitive: bool, 
        is_regex: bool, 
        is_subsitute: bool, 
        substitute: String,
        show_line_number: bool,
        recursiv: bool
    ) -> Result<Config, &'static str> {
            let regex = Regex::new(&query_tmp).unwrap();
            let query = match case_sensitive {
                true => {query_tmp},
                false => {query_tmp.to_lowercase()}
            };
            Ok(Config { query , 
                filename, case_sensitive, is_regex, 
                is_subsitute, substitute, regex, show_line_number, recursiv})
        }

    pub fn set_case_sensitive(&mut self, case_sensitive: bool) {
        if case_sensitive == false {
            self.query = self.query.to_lowercase();
        }
        self.case_sensitive = case_sensitive;
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
            String::from(""), false, true).unwrap();
        assert_eq!(config.query, "test");
    }

    #[test]
    fn test_case_sensitive_config() {
        let config = Config::new(String::from("TeST"), 
            String::from("file"), 
            true, false, false, 
            String::from(""), false, false).unwrap();
        assert_eq!(config.query, "TeST");
    }

}