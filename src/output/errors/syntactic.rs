use super::*;
use std::fmt;

#[derive(Debug)]
pub struct SyntacticError {  
    pub expected: String,
    pub found: String,
    pub position: TokenPosition,
}

impl fmt::Display for SyntacticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ANSI escape code for red text
        let red = "\x1b[31m";
        // ANSI escape code to reset text color
        let reset = "\x1b[0m";
        writeln!(f, "{}{} Syntax Error{}\n\tExpected: {}\n\tFound: {}", self.position, red, reset, self.expected, self.found)
    }
}