pub enum LexicalErrorType {
    InvalidIdentifier,
    InvalidKeyword,
}

pub struct LexicalError<'a> {
    pub error_type: LexicalErrorType,
    pub line_number: u32,
    pub lexeme: &'a str,
}