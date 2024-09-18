use super::*;
use std::fmt;

#[derive(Debug)]
pub struct SyntacticError<'a> {  
    pub expected: String,
    pub found: Token<'a>,
    pub position: TokenPosition,
}

impl<'a> fmt::Display for SyntacticError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ANSI escape code for red text
        let red = "\x1b[31m";
        // ANSI escape code to reset text color
        let reset = "\x1b[0m";
        writeln!(f, "{}{} Syntax Error{}\n\tExpected: {}\n\tFound: {}", self.position, red, reset, self.expected, self.found)
    }
}