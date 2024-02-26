use std::collections::{HashMap, HashSet};

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
        // match opening '('
        if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
            self.tokenizer.get_token()
        {
            // Determine file type
            match self.parse_document_type() {
                // Domain Definition
                Ok(DefinitionTypes::Domain(_)) => {
                    if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                        self.tokenizer.get_token()
                    {
                        // predicate definitions
                        match self.tokenizer.get_token() {
                            Ok(Some(Token::Keyword(KeywordName::Predicates))) => {
                                self.parse_predicates();
                            },

                            _ => {
                                // TODO: better error handling
                                panic!("expected a keyword")
                            }

                        }
                    } else {
                        // TODO: better error handling
                        panic!("expected '('")
                    }
                    Ok(self.symbol_table)
                },
                // Problem Definition
                Ok(DefinitionTypes::Problem(_)) => {
                    if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                        self.tokenizer.get_token()
                    {
                        // match declaration type
                        match self.tokenizer.get_token() {
                            // requirement declaration
                            Ok(Some(Token::Keyword(KeywordName::Requirements))) => {
                                // TODO: handle errors
                                self.parse_requirements();
                            },
                            // objects declaration
                            Ok(Some(Token::Keyword(KeywordName::Objects))) => {
                                // TODO: handle errors
                                self.parse_objects_list(vec![]);
                            },
                            // initial task network declaration
                            Ok(Some(Token::Keyword(KeywordName::HTN))) => {
                                // TODO:
                            },

                            _ => todo!(),
                        }
                    } else {
                        // TODO: better error handling
                        panic!("expected '('")
                    }
                    Ok(self.symbol_table)
                },
                Err(x) => {
                    return Err(x);
                }
            }
        } else {
            // TODO: improve error handling
            panic!("files should start with '('")
        }
    }

    fn parse_document_type(&self) -> Result<DefinitionTypes, SyntacticError> {
        // match keyword 'define'
        if let Ok(Some(Token::Keyword(KeywordName::Define))) = self.tokenizer.get_token() {
            // match '(' after keyword 'define
            if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                self.tokenizer.get_token()
            {
                // match either 'domain' or 'problem'
                match self.tokenizer.get_token() {
                    Ok(Some(Token::Keyword(KeywordName::Domain))) => {
                        // match domain name
                        let next_token = self.tokenizer.get_token();
                        if let Ok(Some(Token::Identifier(domain_name))) = next_token
                        {
                            // match closing paranthesis
                            if let Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) =
                                self.tokenizer.get_token()
                            {
                                return Ok(DefinitionTypes::Domain(domain_name));
                            } else {
                                // TODO: better error handling
                                panic!("expected ')' found ...")
                            }
                        } else {
                            // TODO: better error handling
                            panic!("expected domain name, found blah")
                        }
                    }
                    Ok(Some(Token::Keyword(KeywordName::Problem))) => {
                        // match problem name
                        if let Ok(Some(Token::Identifier(problem_name))) =
                            self.tokenizer.get_token()
                        {
                            // match closing paranthesis
                            if let Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) =
                                self.tokenizer.get_token()
                            {
                                // match '(' for domain name
                                if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                                    self.tokenizer.get_token()
                                {
                                    if let Ok(Some(Token::Keyword(KeywordName::Domain))) =
                                        self.tokenizer.get_token()
                                    {
                                        if let Ok(Some(Token::Identifier(domain_name))) =
                                            self.tokenizer.get_token()
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
                    }
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
    }

    fn parse_requirements(&mut self) -> Result<(), SyntacticError> {
        let token = self.tokenizer.get_token();
        match token {
            Ok(Some(Token::Requirement(RequirementType::TypedObjects))) => {
                self.symbol_table.requirements.insert(RequirementType::TypedObjects);
                self.parse_requirements()
            }
            Ok(Some(Token::Requirement(RequirementType::Hierarchy))) => {
                self.symbol_table.requirements.insert(RequirementType::Hierarchy);
                self.parse_requirements()
            }
            Ok(Some(Token::Requirement(RequirementType::MethodPreconditions))) => {
                self.symbol_table.requirements.insert(RequirementType::MethodPreconditions);
                self.parse_requirements()
            }
            Ok(Some(Token::Requirement(RequirementType::NegativePreconditions))) => {
                self.symbol_table.requirements.insert(RequirementType::NegativePreconditions);
                self.parse_requirements()
            }
            Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                Ok(())
            }
            _ => {
                // TODO: better error handling
                panic!("not a valid requirement")
            }
        }
    }

    // parse a list of objects (may or may not contain typed objects)
    pub fn parse_list(&self) -> TypedList<'a> {
        let mut variables = HashSet::new();
        let mut type_scope = HashSet::new();
        let mut variable_types = HashMap::new();
        loop {
            let token = self.tokenizer.get_token();
            match token {
                Ok(Some(Token::Identifier(variable_name))) => {
                    variables.insert(variable_name);
                    type_scope.insert(variable_name);
                },
                Ok(Some(Token::Punctuator(PunctuationType::Dash))) => {
                    let type_token = self.tokenizer.get_token();
                    if let Ok(Some(Token::Identifier(type_name))) = type_token {
                        for var in type_scope {
                            variable_types.insert(var, type_name);
                        }
                        type_scope = HashSet::new();
                    } else {
                        // TODO: better error handling
                        panic!("expected type")
                    }
                },
                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                    break;
                },
                _ => {
                    // TODO: better error handling
                    panic!("invalid token in the list")
                }
            }
        }
        TypedList {
            variables: variables,
            variable_types: match variable_types.len() {
                0 => None,
                _ => Some(variable_types)
            }
        }
    }
}
