use super::*;

impl<'a> Parser<'a> {
    pub fn parse_initial_tn(&'a self) -> Result<InitialTaskNetwork<'a>, SyntacticError> {
        loop {
            let token = self.tokenizer.get_token();
            match token {
                Ok(Some(Token::Keyword(KeywordName::Parameters))) => {
                    // TODO: there may not be any parrameters
                    if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) = self.tokenizer.get_token() {
                        let params = self.parse_args()?;
                        let init_tn = self.parse_htn()?;
                        return Ok(InitialTaskNetwork {
                            parameters: if params.arguments.len() == 0 {None} else {Some(params)},
                            tn: init_tn
                        });
                    } else {
                        panic!("expected '('")
                    }
                }
                _ => {
                    // TODO: better error handling
                    panic!("ill defined init tn");
                }
            }
        }
    }

    pub fn parse_htn(&'a self) -> Result<HTN<'a>, SyntacticError> {
        let mut subtasks = vec![];
        let mut orderings = vec![];
        let mut constraints = None;

        match self.tokenizer.get_token() {
            Ok(Some(Token::Keyword(KeywordName::Subtasks))) => {
                subtasks = self.parse_subtasks()?;
                loop {
                    match self.tokenizer.get_token() {
                        Ok(Some(Token::Keyword(KeywordName::Ordering))) => {
                            if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) = self.tokenizer.get_token() {
                                match self.tokenizer.get_token() {
                                    Ok(Some(Token::Operator(OperationType::And))) => {
                                        loop {
                                            match self.tokenizer.get_token() {
                                                Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                                                    orderings.extend(self.parse_ordering()?.into_iter());
                                                },
                                                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                                                    break;
                                                },
                                                _ => {
                                                    panic!("unexpected")
                                                }
                                            }
                                        }
                                    },
                                    Ok(Some(Token::Operator(OperationType::LessThan))) => {
                                        if let Ok(Some(Token::Identifier(t1))) = self.tokenizer.get_token() {
                                            loop {
                                                match self.tokenizer.get_token() {
                                                    Ok(Some(Token::Identifier(t2))) => {
                                                        orderings.push((t1, t2));
                                                    }
                                                    Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                                                        break;
                                                    }
                                                    _ => {
                                                        panic!("unexpected");
                                                    }
                                                }
                                            }
                                        } else {
                                            panic!("unexpected")
                                        }
                                    }
                                    _ => {
                                        panic!("unexpected")
                                    }
                                }
                            } else {
                                panic!("unexpected")
                            }
                        },
                        Ok(Some(Token::Keyword(KeywordName::Constraints))) => {
                            constraints = Some(self.parse_constraints()?);
                        },
                        Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                            return Ok(HTN {
                                subtasks,
                                orderings: TaskOrdering::Partial(orderings),
                                constraints,
                            });
                        },
                        p @ _ => {
                            panic!("unexpected {:?}", p)
                        }
                    }
                }
            },
            Ok(Some(Token::Keyword(KeywordName::OrderedSubtasks))) => {
                subtasks = self.parse_subtasks()?;
                match self.tokenizer.get_token() {
                    Ok(Some(Token::Keyword(KeywordName::Constraints))) => {
                        constraints = Some(self.parse_constraints()?);
                        return Ok(HTN {
                            subtasks,
                            orderings: TaskOrdering::Total,
                            constraints,
                        });
                    },
                    Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                        return Ok(HTN {
                            subtasks,
                            orderings: TaskOrdering::Total,
                            constraints,
                        });
                    }
                    _ => {
                        panic!("unexpected")
                    }
                }
            },
            _ => {
                // TODO: better error handling
                panic!("expected subtask definitions")
            }
        }
    }

    fn parse_ordering(&self) -> Result<Vec<(&'a str, &'a str)>, SyntacticError> {
        let mut orderings: Vec<(&str, &str)> = vec![];
        if let Ok(Some(Token::Operator(OperationType::LessThan))) = self.tokenizer.get_token() {
            if let Ok(Some(Token::Identifier(t1))) = self.tokenizer.get_token() {
                loop {
                    match self.tokenizer.get_token() {
                        Ok(Some(Token::Identifier(t2))) => {
                            orderings.push((t1, t2));
                        }
                        Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                            return Ok(orderings);
                        }
                        _ => {
                            panic!("unexpected");
                        }
                    }
                }
            } else {
                panic!("unexpected")
            }
        } else {
            panic!("unexpected")
        }
    }

    fn parse_subtasks(&self) -> Result<Vec<Subtask>, SyntacticError> {
        if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
            self.tokenizer.get_token()
        {
            match self.tokenizer.get_token() {
                Ok(Some(Token::Operator(OperationType::And))) => {
                    let mut subtasks = vec![];
                    loop {
                        match self.tokenizer.get_token() {
                            Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                                return Ok(subtasks);
                            }
                            Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                                subtasks.push(self.parse_subtask()?);
                            }
                            _ => {
                                panic!()
                            }
                        }
                    }
                }
                Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                    return Ok(vec![self.parse_subtask()?]);
                }
                _ => {
                    panic!();
                }
            }
        } else {
            panic!("expected '('")
        }
    }

    // parses a single subtask
    fn parse_subtask(&self) -> Result<Subtask, SyntacticError<'a>> {
        if let Ok(Some(Token::Identifier(id))) = self.tokenizer.get_token() {
            let mut terms = vec![];
            match self.tokenizer.get_token() {
                Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                    if let Ok(Some(Token::Identifier(task))) = self.tokenizer.get_token() {
                        loop {
                            match self.tokenizer.get_token() {
                                Ok(Some(Token::Identifier(term))) => {
                                    terms.push(term);
                                }
                                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                                    if let Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) = self.tokenizer.get_token() {
                                        return Ok(Subtask {
                                            id: Some(id),
                                            task_symbol: task,
                                            terms: terms,
                                        })
                                    } else {
                                        panic!("expected ')'")
                                    }
                                    
                                }
                                _ => {
                                    //TODO:
                                    panic!("unexpected token")
                                }
                            }
                        }
                    } else {
                        // TODO: better error handling
                        panic!("expected subtask name")
                    }
                }
                Ok(Some(Token::Identifier(term))) => {
                    terms.push(term);
                    loop {
                        match self.tokenizer.get_token() {
                            Ok(Some(Token::Identifier(term))) => {
                                terms.push(term);
                            }
                            Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                                return Ok(Subtask {
                                    id: None,
                                    task_symbol: id,
                                    terms: terms,
                                })
                            }
                            _ => {
                                //TODO:
                                panic!("unexpected token")
                            }
                        }
                    }
                }
                _ => {
                    // TODO: better error handling
                    panic!("expected subtask definition")
                }
            }
        } else {
            // TODO:
            panic!("expected task id")
        }
    }

    pub fn parse_constraints(&self) -> Result<Vec<Constraint<'a>>, SyntacticError> {
        if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
            self.tokenizer.get_token()
        {
            let mut constraints = vec![];
            match self.tokenizer.get_token() {
                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                    return Ok(constraints);
                }
                Ok(Some(Token::Operator(OperationType::And))) => loop {
                    match self.tokenizer.get_token() {
                        Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                            constraints.push(self.parse_constraint()?);
                        }
                        Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                            return Ok(constraints);
                        }
                        _ => {
                            panic!("unexpected")
                        }
                    }
                },
                _ => {
                    panic!("unexpected")
                }
            }
        } else {
            panic!("expected '('")
        }
    }

    pub fn parse_constraint(&self) -> Result<Constraint<'a>, SyntacticError> {
        match self.tokenizer.get_token() {
            Ok(Some(Token::Operator(OperationType::Not))) => {
                if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                    self.tokenizer.get_token()
                {
                    if let Ok(Some(Token::Operator(OperationType::Equal))) =
                        self.tokenizer.get_token()
                    {
                        if let Ok(Some(Token::Identifier(t1))) = self.tokenizer.get_token() {
                            if let Ok(Some(Token::Identifier(t2))) = self.tokenizer.get_token() {
                                return Ok(Constraint::NotEqual(t1, t2));
                            } else {
                                panic!("expected task id2");
                            }
                        } else {
                            panic!("expected task id1");
                        }
                    } else {
                        panic!("expected '='");
                    }
                } else {
                    panic!("expected '('")
                }
            }
            Ok(Some(Token::Operator(OperationType::Equal))) => {
                if let Ok(Some(Token::Identifier(t1))) = self.tokenizer.get_token() {
                    if let Ok(Some(Token::Identifier(t2))) = self.tokenizer.get_token() {
                        return Ok(Constraint::Equal(t1, t2));
                    } else {
                        panic!("expected task id2");
                    }
                } else {
                    panic!("expected task id1");
                }
            }
            _ => {
                panic!("unexpected")
            }
        }
    }
}
