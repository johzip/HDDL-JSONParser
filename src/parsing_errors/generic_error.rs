use super::*;
pub enum ParsingError<'a>{
    Lexiacal(LexicalError<'a>),
    Syntactic(SyntacticError<'a>),
    Semantic
}