use super::Token;

pub struct SyntacticError<'a> {  
    pub expected: String,
    pub found: Token<'a>,
    pub line_number: u32,
    pub solution: &'static str
}