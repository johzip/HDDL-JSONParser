use super::*;

impl<'a> Parser<'a> {
    pub fn parse_formula(&'a self) -> Result<Formula, SyntacticError<'a>> {
        match self.tokenizer.get_token() {
            Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                return Ok(Formula::Empty);
            },
            Ok(Token::Punctuator(PunctuationType::LParentheses)) => {
                match self.tokenizer.get_token() {
                    // Not Operation
                    Ok(Token::Operator(OperationType::Not)) => {
                        let formula = self.parse_formula()?;
                        if let Ok(Token::Punctuator(PunctuationType::RParentheses)) = self.tokenizer.get_token() {
                            return Ok(Formula::Not(Box::new(formula)));
                        } else {
                            panic!("expected ')'")
                        }
                    },
                    // And Connector
                    Ok(Token::Operator(OperationType::And)) => {
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
                    Ok(Token::Operator(OperationType::Xor)) => {
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
                    Ok(Token::Operator(OperationType::Or)) => {
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
                    // Single Atom
                    Ok(Token::Identifier(name)) => {
                        let predicate = Predicate {
                            name: name,
                            variables: self.parse_args()?
                        };
                        return Ok(Formula::Atom(predicate));
                    }
                    Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                        return Ok(Formula::Empty);
                    }
                    // TODO: make this better
                    err => {
                        panic!("unexpected token {:?}", err)
                    }
                }
            },
            // TODO: Complete this
            Ok(x) => {
                todo!()
            },
            _ => {
                panic!("error")
            }
        }
    }
}
