use crate::TokenPosition;
use std::fmt;

#[derive(Debug)]
pub enum LexicalErrorType {
    InvalidIdentifier,
    InvalidKeyword,
}

#[derive(Debug)]
pub struct LexicalError<'a> {
    pub error_type: LexicalErrorType,
    pub lexeme: &'a str,
    pub position: TokenPosition,
}

impl<'a> fmt::Display for LexicalError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ANSI escape code for red text
        let red = "\x1b[31m";
        // ANSI escape code to reset text color
        let reset = "\x1b[0m";
        match self.error_type {
            LexicalErrorType::InvalidIdentifier => {
                writeln!(f, "{}{} Invalid Identifier {}{}", self.position, red, reset, self.lexeme)
            }
            LexicalErrorType::InvalidKeyword => {
                writeln!(f, "{}{} Invalid Token {}{}", self.position, red, reset, self.lexeme)
            }
        }
    }
}