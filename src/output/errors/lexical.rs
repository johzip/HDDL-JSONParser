use crate::TokenPosition;

#[derive(Debug)]
pub enum LexicalErrorType {
    InvalidIdentifier,
    InvalidKeyword,
}

#[derive(Debug)]
pub struct LexicalError<'a> {
    pub error_type: LexicalErrorType,
    pub lexeme: &'a str,
    pub position: TokenPosition
}