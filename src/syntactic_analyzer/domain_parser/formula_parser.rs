use super::*;

impl<'a> Parser<'a> {
    pub fn parse_formula(&'a self) -> Result<Formula, SyntacticError<'a>> {
        match self.tokenizer.get_token() {
            Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                return Ok(Formula::Empty);
            },
            Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) => {
                match self.tokenizer.get_token() {
                    // Not Operation
                    Ok(Some(Token::Operator(OperationType::Not))) => {
                        let formula = self.parse_formula()?;
                        if let Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) = self.tokenizer.get_token() {
                            return Ok(Formula::Not(Box::new(formula)));
                        } else {
                            panic!("expected ')'")
                        }
                    },
                    // And Connector
                    Ok(Some(Token::Operator(OperationType::And))) => {
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
                    Ok(Some(Token::Operator(OperationType::Xor))) => {
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
                    Ok(Some(Token::Operator(OperationType::Or))) => {
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
                    Ok(Some(Token::Identifier(name))) => {
                        let predicate = Predicate {
                            name: name,
                            variables: self.parse_args()?
                        };
                        return Ok(Formula::Atom(predicate));
                    }
                    Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                        return Ok(Formula::Empty);
                    }
                    // TODO: make this better
                    err => {
                        panic!("unexpected token {:?}", err)
                    }
                }
            },
            // TODO: Complete this
            Ok(Some(_)) => {
                todo!()
            },
            _ => {
                panic!("error")
            }
        }
    }
}
