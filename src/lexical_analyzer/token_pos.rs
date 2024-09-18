use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct TokenPosition {
    pub line: u32,
    pub column: (u32, Option<u32>)
}

impl <'a> fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ANSI escape code for red text
        let red = "\x1b[31m";
        // ANSI escape code to reset text color
        let reset = "\x1b[0m";
        match self.column.1 {
            Some(end) => {
                write!(f, "{}[Ln {}, Col {}-{}]{}", red, self.line, self.column.0, end, reset)
            }
            None => {
                write!(f, "{}[Ln {}, Col {}]{}", red, self.line, self.column.0, reset)
            }
        }
    }
}