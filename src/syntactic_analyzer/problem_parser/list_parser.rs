use crate::syntactic_analyzer::SyntacticError;
use super::*;

impl <'a> Parser<'a> {
    pub fn parse_args(&self) -> Result<Vec<Variable<'a>>, SyntacticError> {
        let mut objects = vec![];
        let mut result = vec![];
        let mut token = self.tokenizer.get_token();
        loop {
            while let Ok(Token::Identifier(symbol)) = token {
                objects.push(symbol);
                token = self.tokenizer.get_token();
            }
            match token {
                Ok(Token::Punctuator(PunctuationType::Dash)) => {
                    // match type
                    let object_type = self.tokenizer.get_token();
                    token = self.tokenizer.get_token();
                    match object_type {
                        Ok(Token::Identifier(t)) => {
                            for o in objects {
                                result.push(Variable::new(o, Some(t)));
                            }
                            objects = vec![];
                        },
                        Ok(x) => {
                            // TODO: test
                            return Err(SyntacticError {
                                expected: format!("The type of {}", objects.into_iter().clone().collect::<Vec<&'a str>>().join(", ")),
                                found: x,
                                line_number: self.tokenizer.get_line_number(),
                                solution: "Use a type identifier after '-'"
                            });
                        },
                        Err(x) => {
                            // TODO: better error handling
                            panic!("x is not a valid type identifier for ...")
                        }
                    }
                },
                Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                    for o in objects {
                        result.push(Variable::new(o, None));
                    }
                    return Ok(result);
                }
                invalid_token @ _=> {
                    // TODO: better error handling (include invalid token)
                    panic!("expected identifier found ")
                }
            }
        }
    }
}