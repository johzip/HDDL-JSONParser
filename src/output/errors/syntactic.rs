use super::Token;

#[derive(Debug)]
pub struct SyntacticError<'a> {  
    pub expected: String,
    pub found: Token<'a>,
    pub line_number: u32,
}