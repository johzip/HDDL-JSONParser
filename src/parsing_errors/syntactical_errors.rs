use super::Token;

pub struct SyntacticError<'a> {  
    pub expected: Vec<Token<'a>>,
    pub found: &'a str,
    pub line_number: u32,
    pub description: &'static str,
}