use super::TokenType;

#[derive(Debug)]
pub struct SyntacticError<'a> {  
    pub expected: String,
    pub found: TokenType<'a>,
    pub line_number: u32,
}