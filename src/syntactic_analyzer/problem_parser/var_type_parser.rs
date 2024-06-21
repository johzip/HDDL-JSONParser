use super::*;

impl <'a> Parser <'a> {
    pub fn parse_var_types(&self) -> Result<Vec<Variable>, SyntacticError> {
        let mut var_types = vec![];
        loop {
            match self.tokenizer.lookahead() {
                Ok(Some(Token::Identifier(_))) => {
                    var_types.extend(self.parse_args()?);
                }
                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                    let _ = self.tokenizer.get_token();
                    return Ok(var_types);
                }
                _ => {
                    // TODO: implement
                    panic!("unexpected token")
                }
            }
        }
    }
}