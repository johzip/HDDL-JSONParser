#[derive(Debug)]
pub enum LexicalErrorType {
    InvalidIdentifier,
    InvalidKeyword,
}

#[derive(Debug)]
pub struct LexicalError<'a> {
    pub error_type: LexicalErrorType,
    pub line_number: u32,
    pub lexeme: &'a str,
}