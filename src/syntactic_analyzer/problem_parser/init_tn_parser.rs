use crate::parsing_errors::ParsingError;

use super::*;

impl<'a> Parser<'a> {
    pub fn parse_initial_tn(&'a self) -> Result<InitialTaskNetwork<'a>, ParsingError> {
        loop {
            match self.tokenizer.lookahead()? {
                Token::Keyword(KeywordName::Parameters) => {
                    let _ = self.tokenizer.get_token()?;
                    match self.tokenizer.get_token()? {
                        Token::Punctuator(PunctuationType::LParentheses) => {
                            return Ok(InitialTaskNetwork {
                                parameters: Some(self.parse_args()?),
                                tn: self.parse_htn()?,
                            });
                        }
                        token => {
                            let error = SyntacticError {
                                expected: "'(' afer keyword :parameters".to_string(),
                                found: token,
                                line_number: self.tokenizer.get_line_number(),
                            };
                            return Err(ParsingError::Syntactic(error));
                        }
                    }
                }
                Token::Keyword(KeywordName::Subtasks)
                | Token::Keyword(KeywordName::OrderedSubtasks) => {
                    return Ok(InitialTaskNetwork {
                        parameters: None,
                        tn: self.parse_htn()?,
                    });
                }
                token => {
                    let error = SyntacticError {
                        expected: "expected the definition of the initial task network".to_string(),
                        found: token,
                        line_number: self.tokenizer.get_line_number(),
                    };
                    return Err(ParsingError::Syntactic(error));
                }
            }
        }
    }

    pub fn parse_htn(&'a self) -> Result<HTN<'a>, ParsingError> {
        let mut subtasks = vec![];
        let mut orderings = vec![];
        let mut constraints = None;

        match self.tokenizer.get_token()? {
            Token::Keyword(KeywordName::Subtasks) => {
                subtasks = self.parse_subtasks()?;
                loop {
                    match self.tokenizer.get_token()? {
                        Token::Keyword(KeywordName::Ordering) => {
                            match self.tokenizer.get_token()? {
                                Token::Punctuator(PunctuationType::LParentheses) => {
                                    match self.tokenizer.get_token()? {
                                        Token::Operator(OperationType::And) => loop {
                                            match self.tokenizer.get_token()? {
                                                Token::Punctuator(
                                                    PunctuationType::LParentheses,
                                                ) => {
                                                    orderings
                                                        .extend(self.parse_ordering()?.into_iter());
                                                }
                                                Token::Punctuator(
                                                    PunctuationType::RParentheses,
                                                ) => {
                                                    break;
                                                }
                                                _ => {
                                                    // TODO:
                                                    panic!("unexpected")
                                                }
                                            }
                                        },
                                        Token::Operator(OperationType::LessThan) => {
                                            if let Ok(Token::Identifier(t1)) =
                                                self.tokenizer.get_token()
                                            {
                                                loop {
                                                    match self.tokenizer.get_token() {
                                                        Ok(Token::Identifier(t2)) => {
                                                            orderings.push((t1, t2));
                                                        }
                                                        Ok(Token::Punctuator(
                                                            PunctuationType::RParentheses,
                                                        )) => {
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
                                }
                                token => {
                                    let error = SyntacticError {
                                        expected: "'('".to_string(),
                                        found: token,
                                        line_number: self.tokenizer.get_line_number(),
                                    };
                                    return Err(ParsingError::Syntactic(error));
                                }
                            }
                        }
                        Token::Keyword(KeywordName::Constraints) => {
                            constraints = Some(self.parse_constraints()?);
                        }
                        Token::Punctuator(PunctuationType::RParentheses) => {
                            return Ok(HTN {
                                subtasks,
                                orderings: TaskOrdering::Partial(orderings),
                                constraints,
                            });
                        }
                        token => {
                            let error = SyntacticError {
                                expected: "the (potentially empty) ordering constraints of the task network".to_string(),
                                found: token,
                                line_number: self.tokenizer.get_line_number(),
                            };
                            return Err(ParsingError::Syntactic(error));
                        }
                    }
                }
            }
            Token::Keyword(KeywordName::OrderedSubtasks) => {
                subtasks = self.parse_subtasks()?;
                match self.tokenizer.get_token()? {
                    Token::Keyword(KeywordName::Constraints) => {
                        constraints = Some(self.parse_constraints()?);
                        return Ok(HTN {
                            subtasks,
                            orderings: TaskOrdering::Total,
                            constraints,
                        });
                    }
                    Token::Punctuator(PunctuationType::RParentheses) => {
                        return Ok(HTN {
                            subtasks,
                            orderings: TaskOrdering::Total,
                            constraints,
                        });
                    }
                    token => {
                        let error = SyntacticError {
                            expected: "closing ')' after task network definition".to_string(),
                            found: token,
                            line_number: self.tokenizer.get_line_number(),
                        };
                        return Err(ParsingError::Syntactic(error));
                    }
                }
            }
            err => {
                // TODO: better error handling
                panic!("expected subtask definitions, found {:?}", err)
            }
        }
    }

    fn parse_ordering(&self) -> Result<Vec<(&'a str, &'a str)>, SyntacticError> {
        let mut orderings: Vec<(&str, &str)> = vec![];
        if let Ok(Token::Operator(OperationType::LessThan)) = self.tokenizer.get_token() {
            if let Ok(Token::Identifier(t1)) = self.tokenizer.get_token() {
                loop {
                    match self.tokenizer.get_token() {
                        Ok(Token::Identifier(t2)) => {
                            orderings.push((t1, t2));
                        }
                        Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
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
        if let Ok(Token::Punctuator(PunctuationType::LParentheses)) = self.tokenizer.get_token() {
            match self.tokenizer.get_token() {
                Ok(Token::Operator(OperationType::And)) => {
                    let mut subtasks = vec![];
                    loop {
                        match self.tokenizer.get_token() {
                            Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                                return Ok(subtasks);
                            }
                            Ok(Token::Punctuator(PunctuationType::LParentheses)) => {
                                subtasks.push(self.parse_subtask()?);
                            }
                            _ => {
                                panic!()
                            }
                        }
                    }
                }
                Ok(Token::Punctuator(PunctuationType::LParentheses)) => {
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
        if let Ok(Token::Identifier(id)) = self.tokenizer.get_token() {
            let mut terms = vec![];
            match self.tokenizer.get_token() {
                Ok(Token::Punctuator(PunctuationType::LParentheses)) => {
                    if let Ok(Token::Identifier(task)) = self.tokenizer.get_token() {
                        loop {
                            match self.tokenizer.get_token() {
                                Ok(Token::Identifier(term)) => {
                                    terms.push(term);
                                }
                                Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                                    if let Ok(Token::Punctuator(PunctuationType::RParentheses)) =
                                        self.tokenizer.get_token()
                                    {
                                        return Ok(Subtask {
                                            id: Some(id),
                                            task_symbol: task,
                                            terms: terms,
                                        });
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
                Ok(Token::Identifier(term)) => {
                    terms.push(term);
                    loop {
                        match self.tokenizer.get_token() {
                            Ok(Token::Identifier(term)) => {
                                terms.push(term);
                            }
                            Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
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
                Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                    return Ok(Subtask {
                        id: None,
                        task_symbol: id,
                        terms: terms,
                    })
                }
                err => {
                    // TODO: better error handling
                    panic!("expected subtask definition, found {:?}", err)
                }
            }
        } else {
            // TODO:
            panic!("expected task id")
        }
    }

    pub fn parse_constraints(&self) -> Result<Vec<Constraint<'a>>, SyntacticError> {
        if let Ok(Token::Punctuator(PunctuationType::LParentheses)) = self.tokenizer.get_token() {
            let mut constraints = vec![];
            match self.tokenizer.get_token() {
                Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                    return Ok(constraints);
                }
                Ok(Token::Operator(OperationType::And)) => loop {
                    match self.tokenizer.get_token() {
                        Ok(Token::Punctuator(PunctuationType::LParentheses)) => {
                            constraints.push(self.parse_constraint()?);
                        }
                        Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
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
            Ok(Token::Operator(OperationType::Not)) => {
                if let Ok(Token::Punctuator(PunctuationType::LParentheses)) =
                    self.tokenizer.get_token()
                {
                    if let Ok(Token::Operator(OperationType::Equal)) = self.tokenizer.get_token() {
                        if let Ok(Token::Identifier(t1)) = self.tokenizer.get_token() {
                            if let Ok(Token::Identifier(t2)) = self.tokenizer.get_token() {
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
            Ok(Token::Operator(OperationType::Equal)) => {
                if let Ok(Token::Identifier(t1)) = self.tokenizer.get_token() {
                    if let Ok(Token::Identifier(t2)) = self.tokenizer.get_token() {
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
