use self::definition_types::ProblemDefinition;

use super::*;

pub struct Parser<'a> {
    pub tokenizer: &'a LexicalAnalyzer,
    pub symbol_table: SymbolTable<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a LexicalAnalyzer) -> Parser<'a> {
        Parser {
            tokenizer,
            symbol_table: SymbolTable::new(),
        }
    }
    pub fn parse(mut self) -> Result<SymbolTable<'a>, SyntacticError> {
        match self.parse_file_type() {
            // TODO: track domain name
            Ok(DefinitionTypes::Domain(_)) => Ok(self.symbol_table),
            // TODO: track problem name
            Ok(DefinitionTypes::Problem(_)) => {
                self.parse_objects_list(vec![]);
                Ok(self.symbol_table)
            }
            Err(x) => {
                return Err(x);
            }
        }
    }

    fn parse_file_type(&self) -> Result<DefinitionTypes, SyntacticError> {
        // match opening '('
        if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) = self.tokenizer.get_token() {
            // match keyword 'define'
            if let Ok(Some(Token::Keyword(KeywordName::Define))) = self.tokenizer.get_token() {
                // match '(' after keyword 'define
                if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) = self.tokenizer.get_token() {
                    // match either 'domain' or 'problem'
                    match self.tokenizer.get_token() {
                        Ok(Some(Token::Keyword(KeywordName::Domain))) => {
                            // match domain name
                            if let Ok(Some(Token::Identifier(domain_name))) = self.tokenizer.get_token() {
                                // match closing paranthesis
                                if let Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) = self.tokenizer.get_token() {
                                    return Ok(DefinitionTypes::Domain(domain_name));
                                } else {
                                    // TODO: better error handling
                                    panic!("expected ')' found ...")
                                }
                            } else {
                                // TODO: better error handling
                                panic!("expected domain name, found blah")
                            }
                        },
                        Ok(Some(Token::Keyword(KeywordName::Problem))) => {
                            // match problem name
                            if let Ok(Some(Token::Identifier(problem_name))) = self.tokenizer.get_token() {
                                // match closing paranthesis
                                if let Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) = self.tokenizer.get_token() {
                                    // match '(' for domain name
                                    if let Ok(Some(Token::Punctuator(
                                        PunctuationType::LParentheses,
                                    ))) = self.tokenizer.get_token()
                                    {
                                        if let Ok(Some(Token::Keyword(KeywordName::Domain))) = self.tokenizer.get_token() {
                                            if let Ok(Some(Token::Identifier(domain_name))) = self.tokenizer.get_token()
                                            {
                                                if let Ok(Some(Token::Punctuator(
                                                    PunctuationType::RParentheses,
                                                ))) = self.tokenizer.get_token()
                                                {
                                                    return Ok(DefinitionTypes::Problem(
                                                        ProblemDefinition {
                                                            domain_name: domain_name,
                                                            problem_name: problem_name,
                                                        },
                                                    ));
                                                } else {
                                                    // TODO:
                                                    panic!("problem def name is not closed");
                                                }
                                            } else {
                                                // TODO:
                                                panic!("expected domain name")
                                            }
                                        } else {
                                            // TODO:
                                            panic!("expected ':domain")
                                        }
                                    } else {
                                        // TODO:
                                        panic!("expected")
                                    }
                                } else {
                                    // TODO: better error handling
                                    panic!("expected ')' found ...")
                                }
                            } else {
                                // TODO: better error handling
                                panic!("expected problem name, found blah")
                            }
                        },
                        _ => {
                            panic!("expected keyword problem/domain, found somethign else")
                        }
                    }
                } else {
                    // TODO: better error handling
                    panic!("expected '(' after define")
                }
            } else {
                // TODO:
                panic!("expected keyword 'define', but found something else")
            }
        } else {
            // TODO: improve error handling
            panic!("files should start with '('")
        }
    }
}
