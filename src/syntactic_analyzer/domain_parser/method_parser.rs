use crate::parsing_errors::ParsingError;

use super::*;

impl<'a> Parser<'a> {
    pub fn parse_method(&'a self) -> Result<Method<'a>, ParsingError> {
        match self.tokenizer.get_token()? {
            Token::Identifier(method_name) => {
                match self.tokenizer.get_token()? {
                    Token::Keyword(KeywordName::Parameters) => {
                        match self.tokenizer.get_token()? {
                            Token::Punctuator(PunctuationType::LParentheses) => {
                                let params = self.parse_args()?;
                                match self.tokenizer.get_token()? {
                                    Token::Keyword(KeywordName::Task) => {
                                        match self.tokenizer.get_token()? {
                                            Token::Punctuator(PunctuationType::LParentheses) => {
                                                match self.tokenizer.get_token()? {
                                                    Token::Identifier(task_name) => {
                                                        let terms = self.parse_args()?;
                                                        match self.tokenizer.lookahead()? {
                                                            Token::Keyword(KeywordName::Precondition) => {
                                                                // skip "precondition" keyword
                                                                let _ = self.tokenizer.get_token();
                                                                let precondition = self.parse_formula()?;
                                                                let tn = self.parse_htn()?;
                                                                return Ok(Method {
                                                                    name: method_name,
                                                                    params,
                                                                    task_name: task_name,
                                                                    task_terms: terms,
                                                                    precondition: Some(precondition),
                                                                    tn,
                                                                });
                                                            }
                                                            Token::Keyword(KeywordName::Subtasks)
                                                            | Token::Keyword(
                                                                KeywordName::OrderedSubtasks,
                                                            ) => {
                                                                let tn = self.parse_htn()?;
                                                                return Ok(Method {
                                                                    name: method_name,
                                                                    params,
                                                                    task_name: task_name,
                                                                    task_terms: terms,
                                                                    precondition: None,
                                                                    tn,
                                                                });
                                                            }
                                                            token => {
                                                                let error = SyntacticError {
                                                            expected: format!(
                                                                "Either preconditions for {} or its decomposition",
                                                                method_name
                                                            )
                                                            .to_string(),
                                                            found: token,
                                                            line_number: self.tokenizer.get_line_number(),
                                                        };
                                                                return Err(ParsingError::Syntactic(error));
                                                            }
                                                        }
                                                    }
                                                    token => {
                                                        let error = SyntacticError {
                                                            expected: format!("The task that method {} decomposes", method_name).to_string(),
                                                            found: token,
                                                            line_number: self.tokenizer.get_line_number(),
                                                        };
                                                        return Err(ParsingError::Syntactic(error));
                                                    }
                                                }
                                                
                                            }
                                            token => {
                                                let error = SyntacticError {
                                                    expected: "'(' after keyword :task".to_string(),
                                                    found: token,
                                                    line_number: self.tokenizer.get_line_number(),
                                                };
                                                return Err(ParsingError::Syntactic(error));
                                            }
                                        }
                                    }
                                    token => {
                                        let error = SyntacticError {
                                            expected: "keyword :task".to_string(),
                                            found: token,
                                            line_number: self.tokenizer.get_line_number(),
                                        };
                                        return Err(ParsingError::Syntactic(error));
                                    }
                                }
                            }
                            token => {
                                let error = SyntacticError {
                                    expected: "'(' after keyword :parameters".to_string(),
                                    found: token,
                                    line_number: self.tokenizer.get_line_number(),
                                };
                                return Err(ParsingError::Syntactic(error));
                            }
                        }
                    }
                    token => {
                        let error = SyntacticError {
                            expected: format!("The parameters of method {} ", method_name)
                                .to_string(),
                            found: token,
                            line_number: self.tokenizer.get_line_number(),
                        };
                        return Err(ParsingError::Syntactic(error));
                    }
                }
            }
            token => {
                let error = SyntacticError {
                    expected: "method name".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }
    }
}
