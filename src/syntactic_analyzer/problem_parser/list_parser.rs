use crate::syntactic_analyzer::SyntacticError;
use super::*;

impl <'a> Parser<'a> {
    pub fn parse_list(&self) -> Result<TypedList<'a>, SyntacticError> {
        let mut objects = vec![];
        let mut result = TypedList {arguments: vec![]};
        let mut token = self.tokenizer.get_token();
        loop {
            while let Ok(Some(Token::Identifier(symbol))) = token {
                objects.push(symbol);
                token = self.tokenizer.get_token();
            }
            match token {
                Ok(Some(Token::Punctuator(PunctuationType::Dash))) => {
                    // match type
                    let object_type = self.tokenizer.get_token();
                    token = self.tokenizer.get_token();
                    match object_type {
                        Ok(Some(Token::Identifier(t))) => {
                            for o in objects {
                                result.arguments.push(Variable::new(o, Some(t)));
                            }
                            objects = vec![];
                        },
                        Ok(Some(x)) => {
                            // TODO: better error handling
                            panic!("expected a type name but found {:?}", x)
                        },
                        Ok(None) => {
                            // TODO: better error handling
                            panic!("expected object type before end of the file")
                        }
                        Err(x) => {
                            // TODO: better error handling
                            panic!("x is not a valid type identifier for ...")
                        }
                    }
                },
                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                    for o in objects {
                        result.arguments.push(Variable::new(o, None));
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