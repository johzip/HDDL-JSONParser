use std::error;

use super::*;

impl <'a> Parser <'a> {
    pub fn parse_compound_task(&self) -> Result<Task, SyntacticError<'a>>{
        if let Ok(Some(Token::Identifier(task_name))) = self.tokenizer.get_token() {
            if let Ok(Some(Token::Keyword(KeywordName::Parameters))) = self.tokenizer.get_token() {
                if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) = self.tokenizer.get_token() {
                    let list = self.parse_args();
                    match list {
                        Ok(parameters) => {
                            return Ok(Task::new(task_name, parameters));
                        },
                        Err(error) => {
                            panic!("err");
                        }
                    }
                } else {
                    // TODO: better error handling
                    panic!("mising '(' after parameters")
                }

            } else {
                // TODO: better error handling
                panic!("expected keyword :parameters")
            }
        } else {
            // TODO: better error handling
            panic!("expected compound task name")
        }
    }
}