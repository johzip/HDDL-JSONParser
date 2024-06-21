use super::*;

impl<'a> Parser<'a> {
    pub fn parse_method(&'a self) -> Result<Method<'a>, SyntacticError> {
        if let Ok(Some(Token::Identifier(method_name))) = self.tokenizer.get_token() {
            if let Ok(Some(Token::Keyword(KeywordName::Parameters))) = self.tokenizer.get_token() {
                if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                    self.tokenizer.get_token()
                {
                    let params = self.parse_args()?;
                    if let Ok(Some(Token::Keyword(KeywordName::Task))) = self.tokenizer.get_token()
                    {
                        if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                            self.tokenizer.get_token()
                        {
                            if let Ok(Some(Token::Identifier(task_name))) =
                                self.tokenizer.get_token()
                            {
                                let terms = self.parse_args()?;
                                // TODO: parse method preconditions if available
                                match self.tokenizer.lookahead() {
                                    Ok(Some(Token::Keyword(KeywordName::Precondition))) => {
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
                                    Ok(Some(Token::Keyword(KeywordName::Subtasks))) |
                                    Ok(Some(Token::Keyword(KeywordName::OrderedSubtasks))) => {
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
                                    _ => {
                                        panic!("expected either prec or htn")
                                    }
                                }
                                
                            } else {
                                panic!("expected task name")
                            }
                        } else {
                            panic!("expected '('")
                        }
                    } else {
                        panic!("expected keyword task")
                    }
                } else {
                    panic!("expected '('")
                }
            } else {
                panic!("expected parameters keyord")
            }
        } else {
            panic!("expected method name")
        }
    }
}
