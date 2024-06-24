use super::*;
pub enum ParsingError<'a>{
    Lexiacal(LexicalError<'a>),
    Syntactic(SyntacticError<'a>),
    Semantic
}

impl <'a> From<LexicalError<'a>> for ParsingError<'a> {
    fn from(value: LexicalError<'a>) -> Self {
        ParsingError::Lexiacal(value)
    }
}

impl <'a> From<SyntacticError<'a>> for ParsingError<'a> {
    fn from(value: SyntacticError<'a>) -> Self {
        ParsingError::Syntactic(value)
    }
}