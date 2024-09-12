use super::*;

impl <'a> Parser <'a> {
    pub fn parse_task(&'a self) -> Result<Task, ParsingError<'a>>{
        match self.tokenizer.get_token()? {
            TokenType::Identifier(task_name) => {
                match self.tokenizer.get_token()? {
                    TokenType::Keyword(KeywordName::Parameters) => {
                        match self.tokenizer.get_token()? {
                            TokenType::Punctuator(PunctuationType::LParentheses) => {
                                return Ok(Task::new(task_name, self.parse_args()?))
                            }
                            token => {
                                let error = SyntacticError{
                                    expected: "'(' after :parameters".to_string(),
                                    found: token,
                                    line_number: self.tokenizer.get_line_number(),
                                };
                                return Err(ParsingError::Syntactic(error));
                            }
                        }
                    }
                    token => {
                        let error = SyntacticError{
                            expected: format!("a (potentially empty) list of parameters after defininig {}", task_name).to_string(),
                            found: token,
                            line_number: self.tokenizer.get_line_number(),
                        };
                        return Err(ParsingError::Syntactic(error));
                    }
                }
            }
            token => {
                let error = SyntacticError{
                    expected: "a task/action name (identifier)".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }
    }
}