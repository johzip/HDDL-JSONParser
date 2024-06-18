use super::*;

impl <'a> Parser <'a> {
    pub fn parse_action(&'a self) -> Result<Action, SyntacticError<'a>> {
        let task = self.parse_task()?;
        let mut preconditions = None;
        let mut effects = None;
        // Parse Preconditions
        match self.tokenizer.get_token() {
            Ok(Some(Token::Keyword(KeywordName::Precondition))) => {
                match self.tokenizer.get_token() {
                    Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                        preconditions = Some(self.parse_formula()?);
                    },
                    _ => {
                        panic!("expected '('")
                    }  
                }
            },
            _ => {
                panic!("expected keyword ':precondition'")
            }            
        }
        // Skip closing ')' for precondition
        if let Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) = self.tokenizer.get_token() {}
        else {
            panic!("expected ')'")
        }
        // Parse Effects
        match self.tokenizer.get_token() {
            Ok(Some(Token::Keyword(KeywordName::Effect))) => {
                match self.tokenizer.get_token() {
                    Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                        effects = Some(self.parse_formula()?);
                    },
                    _ => {
                        panic!("expected '('")
                    }  
                }
            },
            _ => {
                panic!("expected keyword ':effects'")
            }            
        }
        Ok(Action {
            name: task.name,
            parameters: task.parameters,
            preconditions: preconditions.unwrap(),
            effects: effects.unwrap()
        })
    }
}