use crate::syntactic_analyzer::SyntacticError;
use super::*;

impl <'a> Parser<'a>{
    // TODO: Remove the recursion and use the symbol table instead
    pub fn parse_objects_list(&mut self, objects: Vec<&'a str>) -> Result<(), SyntacticError> {
        let token = self.tokenizer.get_token();
        match token {
            Ok(Some(object)) => {
                match object {
                    Token::Identifier(symbol) => {
                        let mut objects = objects;
                        objects.push(symbol);
                        self.parse_objects_list(objects)
                    },
                    Token::Punctuator(PunctuationType::Dash) => {
                        // match type
                        let object_type = self.tokenizer.get_token();
                        match object_type {
                            Ok(Some(Token::Identifier(x))) => {
                                // TODO: add objects to symbol table with their type
                                for o in objects {
                                    match self.symbol_table.add_typed_object(o, x) {
                                        Ok(()) => {},
                                        Err(_) => {
                                            // TODO: invoke error

                                        }
                                    }
                                }
                                self.parse_objects_list(vec![])
                            },
                            Ok(Some(x)) => {
                                // TODO: better error handling
                                panic!("expected a type name but found...")
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
                    }
                    Token::Punctuator(PunctuationType::RParentheses) => {
                        for o in objects {
                            match self.symbol_table.add_object(o) {
                                Ok(_) => {},
                                Err(_) => {
                                    // TODO: better error handling
                                    panic!("error")
                                }
                            }
                        }
                        return Ok(());
                    }
                    invalid_token @ _=> {
                        // TODO: better error handling (include invalid token)
                        panic!("expected identifier found ")
                    }
                }
            },
            Ok(None) => {
                // TODO: better error handling
                panic!("reached end of file while parsing objects in the problem description")
            }
            Err(error) => {
                match error.error_type {
                    LexicalErrorType::InvalidIdentifier => {
                        // TODO: better error handling
                        panic!("expected object/type, found ':' which is reserved for keywords")
                    },
                    LexicalErrorType::InvalidKeyword => {
                        // TODO: better error handling
                        panic!("invalid keyword")
                    }
                }
            }
        }
    }
}