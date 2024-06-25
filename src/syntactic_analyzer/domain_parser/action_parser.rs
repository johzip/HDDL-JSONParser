use super::*;

impl <'a> Parser <'a> {
    pub fn parse_action(&'a self) -> Result<Action, SyntacticError<'a>> {
        let task = self.parse_task()?;
        let mut preconditions = None;
        let mut effects = None;
        // Parse Preconditions
        match self.tokenizer.get_token() {
            Ok(Token::Keyword(KeywordName::Precondition)) => {
                preconditions = Some(self.parse_formula()?);
            },
            _ => {
                panic!("expected keyword ':precondition'")
            }            
        }
        // Parse Effects
        match self.tokenizer.get_token() {
            Ok(Token::Keyword(KeywordName::Effect)) => {
                effects = Some(self.parse_formula()?);
            },
            _ => {
                panic!("expected keyword ':effects'")
            }            
        }
        // skip action block's closing parantheses
        if let Ok(Token::Punctuator(PunctuationType::RParentheses)) = self.tokenizer.get_token() {}
        Ok(Action {
            name: task.name,
            parameters: task.parameters,
            preconditions: preconditions.unwrap(),
            effects: effects.unwrap()
        })
    }
}