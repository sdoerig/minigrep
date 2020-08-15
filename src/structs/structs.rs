use std::fmt;


pub struct MatchedLine {
    pub matched: bool,
    pub line: String
}

impl fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}



pub struct PrintableWithFileNameLineNumber<'a>{
    pub matched: MatchedLine,
    pub filename: &'a str,
    pub line_number: usize,
    pub show_line_number: bool,
    pub show_file_name: bool
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
        if self.show_file_name && self.show_line_number {
            write!(f, "{}: {}: {}", self.filename, self.line_number, self.matched)
        } else if !self.show_file_name && self.show_line_number {
            write!(f, "{}: {}", self.line_number, self.matched)
        } else if self.show_file_name && !self.show_line_number {
            write!(f, "{}: {}", self.filename, self.matched)
        } else {
            write!(f, "{}", self.matched)
        } 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printable_with_filename_and_linenumber() {
        let matched = MatchedLine{matched: true, line: String::from("testLine")};
        let pwfn = PrintableWithFileNameLineNumber{line_number: 123, filename: &String::from("fileName"), show_line_number: true, show_file_name: true, matched };
        assert_eq!("fileName: 123: testLine",
           format!("{}", pwfn));
    }

    #[test]
    fn printable_linenumber() {
        let matched = MatchedLine{matched: true, line: String::from("testLine")};
        let pwfn = PrintableWithFileNameLineNumber{line_number: 123, filename: &String::from("fileName"), show_line_number: true, show_file_name: false, matched };
        assert_eq!("123: testLine",
           format!("{}", pwfn));
    }
    #[test]
    fn printable_filename() {
        let matched = MatchedLine{matched: true, line: String::from("testLine")};
        let pwfn = PrintableWithFileNameLineNumber{line_number: 123, filename: &String::from("fileName"), show_line_number: false, show_file_name: true, matched };
        assert_eq!("fileName: testLine",
           format!("{}", pwfn));
    }
    #[test]
    fn printable_line_only() {
        let matched = MatchedLine{matched: true, line: String::from("testLine")};
        let pwfn = PrintableWithFileNameLineNumber{line_number: 123, filename: &String::from("fileName"), show_line_number: false, show_file_name: false, matched };
        assert_eq!("testLine",
           format!("{}", pwfn));
    }

}

