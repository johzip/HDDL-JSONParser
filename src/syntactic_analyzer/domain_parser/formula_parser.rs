use super::*;

impl<'a> Parser<'a> {
    pub fn parse_formula(&'a self) -> Result<Formula, ParsingError<'a>> {
        match self.tokenizer.get_token()? {
            TokenType::Punctuator(PunctuationType::RParentheses) => {
                return Ok(Formula::Empty);
            },
            TokenType::Punctuator(PunctuationType::LParentheses) => {
                match self.tokenizer.get_token()? {
                    // Not Operation
                    TokenType::Operator(OperationType::Not) => {
                        let formula = self.parse_formula()?;
                        match self.tokenizer.get_token()? {
                            TokenType::Punctuator(PunctuationType::RParentheses) => {
                                return Ok(Formula::Not(Box::new(formula)));
                            }
                            token => {
                                let error = SyntacticError{
                                    expected: "closing the not operator with ')'".to_string(),
                                    found: token,
                                    line_number: self.tokenizer.get_line_number(),
                                };
                                return Err(ParsingError::Syntactic(error));
                            }
                        }
                    },
                    // And Connector
                    TokenType::Operator(OperationType::And) => {
                        let mut expressions = vec![];
                        loop {
                            let formula = self.parse_formula()?;
                            if let Formula::Empty = formula {
                                return Ok(Formula::And(expressions));
                            } else {
                                expressions.push(Box::new(formula));
                            }
                        }        
                    },
                    // Xor Connector
                    TokenType::Operator(OperationType::Xor) => {
                        let mut expressions = vec![];
                        loop {
                            let formula = self.parse_formula()?;
                            if let Formula::Empty = formula {
                                return Ok(Formula::Xor(expressions));
                            } else {
                                expressions.push(Box::new(formula));
                            }
                        }        
                    },
                    // Or Connector
                    TokenType::Operator(OperationType::Or) => {
                        let mut expressions = vec![];
                        loop {
                            let formula = self.parse_formula()?;
                            if let Formula::Empty = formula {
                                return Ok(Formula::Or(expressions));
                            } else {
                                expressions.push(Box::new(formula));
                            }
                        }        
                    },
                    // Equality
                    TokenType::Operator(OperationType::Equal) => {
                        match self.tokenizer.get_token()? {
                            TokenType::Identifier(p1) => {
                                match self.tokenizer.get_token()? {
                                    TokenType::Identifier(p2) => {
                                        match self.tokenizer.get_token()? {
                                            TokenType::Punctuator(PunctuationType::RParentheses) => {
                                                return Ok(Formula::Equals(p1, p2));
                                            }
                                            token => {
                                                let error = SyntacticError{
                                                    expected: "equality's closing parenthesis".to_string(),
                                                    found: token,
                                                    line_number: self.tokenizer.get_line_number(),
                                                };
                                                return Err(ParsingError::Syntactic(error));
                                            }
                                        }
                                    }
                                    token => {
                                        let error = SyntacticError{
                                            expected: "right hand side of the equality".to_string(),
                                            found: token,
                                            line_number: self.tokenizer.get_line_number(),
                                        };
                                        return Err(ParsingError::Syntactic(error));
                                    }
                                }
                            }
                            token => {
                                let error = SyntacticError{
                                    expected: "left hand side of the equality".to_string(),
                                    found: token,
                                    line_number: self.tokenizer.get_line_number(),
                                };
                                return Err(ParsingError::Syntactic(error));
                            }
                        }
                    }
                    // Universal Quantifier
                    TokenType::Operator(OperationType::ForAll) => {
                        match self.tokenizer.get_token()? {
                            TokenType::Punctuator(PunctuationType::LParentheses) => {
                                let params = self.parse_args()?;
                                let expression = Box::new(self.parse_formula()?);
                                match self.tokenizer.get_token()? {
                                    TokenType::Punctuator(PunctuationType::RParentheses) => {
                                        return Ok(Formula::ForAll(params, expression));
                                    }
                                    token => {
                                        let error = SyntacticError{
                                            expected: "')' to close the forall statement".to_string(),
                                            found: token,
                                            line_number: self.tokenizer.get_line_number(),
                                        };
                                        return Err(ParsingError::Syntactic(error));
                                    }
                                }
                            }
                            token => {
                                let error = SyntacticError{
                                    expected: "'(' after forall keyword".to_string(),
                                    found: token,
                                    line_number: self.tokenizer.get_line_number(),
                                };
                                return Err(ParsingError::Syntactic(error));
                            }
                        }
                    }
                    // Existential Quantifier
                    TokenType::Operator(OperationType::Exists) => {
                        // TODO:
                        todo!()
                    }
                    // Single Atom
                    TokenType::Identifier(name) => {
                        let predicate = Predicate {
                            name: name,
                            variables: self.parse_args()?
                        };
                        return Ok(Formula::Atom(predicate));
                    }
                    TokenType::Punctuator(PunctuationType::RParentheses) => {
                        return Ok(Formula::Empty);
                    }
                    token => {
                        let error = SyntacticError{
                            expected: "a boolean formula".to_string(),
                            found: token,
                            line_number: self.tokenizer.get_line_number(),
                        };
                        return Err(ParsingError::Syntactic(error));
                    }
                }
            },
            token => {
                let error = SyntacticError {
                    expected: "a (potentially empty) boolean formula definition".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            },
        }
    }
}
