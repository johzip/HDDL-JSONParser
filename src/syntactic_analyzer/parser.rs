use std::collections::{HashMap, HashSet};

use self::definition_types::ProblemDefinition;

use super::*;

pub struct Parser<'a> {
    pub tokenizer: &'a LexicalAnalyzer,
    pub syntax_tree: SyntaxTree<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a LexicalAnalyzer) -> Parser<'a> {
        Parser {
            tokenizer,
            syntax_tree: SyntaxTree::new(),
        }
    }
    pub fn parse(mut self) -> Result<SyntaxTree<'a>, SyntacticError> {
        // match opening '('
        if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
            self.tokenizer.get_token()
        {
            // Determine file type
            match self.parse_document_type() {
                // Domain Definition
                Ok(DefinitionType::Domain(_)) => {
                    if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                        self.tokenizer.get_token()
                    {
                        match self.tokenizer.get_token() {
                            // predicate definitions
                            Ok(Some(Token::Keyword(KeywordName::Predicates))) => {
                                self.parse_predicates();
                            },
                            // compund task definitions
                            Ok(Some(Token::Keyword(KeywordName::Task))) => {
                                self.parse_compound_task();
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
                    Ok(self.syntax_tree)
                },
                // Problem Definition
                Ok(DefinitionType::Problem(_)) => {
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
                    Ok(self.syntax_tree)
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

    fn parse_document_type(&self) -> Result<DefinitionType, SyntacticError> {
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
                                return Ok(DefinitionType::Domain(domain_name));
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
                                                return Ok(DefinitionType::Problem(
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
                self.syntax_tree.add_requirement(RequirementType::TypedObjects);
                self.parse_requirements()
            }
            Ok(Some(Token::Requirement(RequirementType::Hierarchy))) => {
                self.syntax_tree.add_requirement(RequirementType::Hierarchy);
                self.parse_requirements()
            }
            Ok(Some(Token::Requirement(RequirementType::MethodPreconditions))) => {
                self.syntax_tree.add_requirement(RequirementType::MethodPreconditions);
                self.parse_requirements()
            }
            Ok(Some(Token::Requirement(RequirementType::NegativePreconditions))) => {
                self.syntax_tree.add_requirement(RequirementType::NegativePreconditions);
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
        let mut variables = vec![];
        let mut type_scope = HashSet::new();
        let mut variable_types = HashMap::new();
        loop {
            let token = self.tokenizer.get_token();
            match token {
                Ok(Some(Token::Identifier(variable_name))) => {
                    variables.push(variable_name);
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
