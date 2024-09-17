use super::*;

#[derive(Debug)]
pub struct SyntacticError<'a> {  
    pub expected: String,
    pub found: Token<'a>,
    pub position: TokenPosition,
}