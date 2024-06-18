use super::*;

impl <'a> Parser <'a> {
    pub fn parse_formula(&self) -> Result<Formula, SyntacticError<'a>> {
        match self.tokenizer.get_token() {
            Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                return Ok(Formula::Empty);
            },
            // TODO: Complete this
            Ok(Some(_)) => {
                todo!()
            },
            _ => {
                panic!("error")
            }
        }
    }
}