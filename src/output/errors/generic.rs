use super::*;

#[derive(Debug)]
pub enum ParsingError<'a>{
    Lexiacal(LexicalError<'a>),
    Syntactic(SyntacticError<'a>),
    Semantic(SemanticErrorType<'a>)
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

impl <'a> From<SemanticErrorType<'a>> for ParsingError<'a> {
    fn from(value: SemanticErrorType<'a>) -> Self {
        ParsingError::Semantic(value)
    }
}

impl <'a> std::fmt::Display for ParsingError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lexiacal(error) => write!(f, "{}", error.to_string()),
            Self::Syntactic(error) => write!(f, "{}", error.to_string()),
            Self::Semantic(error) => write!(f, "{}", error.to_string())
        }
    }
}
