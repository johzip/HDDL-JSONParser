use super::Token;

pub enum SyntacticErrorType<'a>{
    UnexpectedToken(UnexpectedTokenError<'a>),
}

pub struct SyntacticError<'a> {
    error_type: SyntacticErrorType<'a>,
    line_number: u32,
}

pub struct UnexpectedTokenError<'a> {
    context: &'static str,
    expected: Token<'a>,
    found: Token<'a>,
}