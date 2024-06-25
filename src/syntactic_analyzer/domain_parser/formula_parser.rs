use crate::parsing_errors::ParsingError;

use super::*;

impl<'a> Parser<'a> {
    pub fn parse_formula(&'a self) -> Result<Formula, ParsingError<'a>> {
        match self.tokenizer.get_token()? {
            Token::Punctuator(PunctuationType::RParentheses) => {
                return Ok(Formula::Empty);
            },
            Token::Punctuator(PunctuationType::LParentheses) => {
                match self.tokenizer.get_token()? {
                    // Not Operation
                    Token::Operator(OperationType::Not) => {
                        let formula = self.parse_formula()?;
                        match self.tokenizer.get_token()? {
                            Token::Punctuator(PunctuationType::RParentheses) => {
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
                    Token::Operator(OperationType::And) => {
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
                    Token::Operator(OperationType::Xor) => {
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
                    Token::Operator(OperationType::Or) => {
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
                    Token::Identifier(name) => {
                        let predicate = Predicate {
                            name: name,
                            variables: self.parse_args()?
                        };
                        return Ok(Formula::Atom(predicate));
                    }
                    Token::Punctuator(PunctuationType::RParentheses) => {
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
            // TODO: Complete this
            token => {
                todo!()
            },
        }
    }
}
