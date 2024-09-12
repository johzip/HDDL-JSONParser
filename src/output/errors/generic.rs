use super::*;

#[derive(Debug)]
pub enum ParsingError<'a>{
    Lexiacal(LexicalError<'a>),
    Syntactic(SyntacticError<'a>),
    Semantic(SemanticError<'a>)
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
