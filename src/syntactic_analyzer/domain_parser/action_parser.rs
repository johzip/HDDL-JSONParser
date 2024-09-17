use super::*;

impl <'a> Parser <'a> {
    pub fn parse_action(&'a self) -> Result<Action, ParsingError<'a>> {
        let task = self.parse_task()?;
        let mut preconditions = None;
        let mut effects = None;
        // Parse Preconditions
        match self.tokenizer.lookahead()? {
            Token::Keyword(KeywordName::Precondition) => {
                // skip precondition keyword
                let _ = self.tokenizer.get_token();
                preconditions = Some(self.parse_formula()?);
            },
            // the action has no precondition
            Token::Keyword(KeywordName::Effect) => {}
            // undefined sequenec 
            token => {
                let error = SyntacticError{
                    expected: format!("(potentially empty) preconditions of {}", task.name).to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error))
            }            
        }
        // Parse Effects
        match self.tokenizer.lookahead()? {
            Token::Keyword(KeywordName::Effect) => {
                // skip effects keyword
                let _ = self.tokenizer.get_token();
                effects = Some(self.parse_formula()?);
            },
            // action has no effects
            Token::Punctuator(PunctuationType::RParentheses) => {}
            token => {
                let error = SyntacticError{
                    expected: format!("(potentially empty) effects of {}", task.name).to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error))
            }            
        }
        // skip action block's closing parantheses
        match self.tokenizer.get_token()? {
            Token::Punctuator(PunctuationType::RParentheses) => {},
            token => {
                let error = SyntacticError {
                    expected: format!("closing the scope of {} using ')'", task.name).to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }

        Ok(Action {
            name: task.name,
            parameters: task.parameters,
            preconditions: preconditions,
            effects: effects
        })
    }
}