use super::Token;

pub enum SyntacticErrorType<'a>{
    UnexpectedToken(UnexpectedTokenError<'a>),
    EOF
}

pub struct SyntacticError<'a> {
    pub error_type: SyntacticErrorType<'a>,
    pub line_number: u32,
}

pub struct UnexpectedTokenError<'a> {
    pub context: &'static str,
    pub expected: Token<'a>,
    pub found: &'a str,
}