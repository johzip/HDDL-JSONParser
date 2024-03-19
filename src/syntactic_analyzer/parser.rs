use std::collections::{HashMap, HashSet};

use self::definition_types::ProblemDefinition;

use super::*;

pub struct Parser<'a> {
    pub tokenizer: &'a LexicalAnalyzer,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a LexicalAnalyzer) -> Parser<'a> {
        Parser { tokenizer }
    }
    pub fn parse(&'a self) -> Result<SyntaxTree<'a>, SyntacticError<'a>> {
        let mut syntax_tree = SyntaxTree::new();
        // match opening '('
        if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
            self.tokenizer.get_token()
        {
            // Determine file type
            match self.parse_document_type()? {
                // Domain Definition
                DefinitionType::Domain(_) => {
                    if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                        self.tokenizer.get_token()
                    {
                        match self.tokenizer.get_token() {
                            // predicate definitions
                            Ok(Some(Token::Keyword(KeywordName::Predicates))) => {
                                let predicates = self.parse_predicates()?;
                                for predicate in predicates {
                                    syntax_tree.add_predicate(predicate);
                                }
                            }
                            // compund task definitions
                            Ok(Some(Token::Keyword(KeywordName::Task))) => {
                                let task = self.parse_compound_task()?;
                                syntax_tree.add_compound_task(task);
                            }
                            _ => {
                                // TODO: better error handling
                                panic!("expected a keyword")
                            }
                        }
                    } else {
                        // TODO: better error handling
                        panic!("expected '('")
                    }
                    Ok(syntax_tree)
                }
                // Problem Definition
                DefinitionType::Problem(_) => {
                    if let Ok(Some(Token::Punctuator(PunctuationType::LParentheses))) =
                        self.tokenizer.get_token()
                    {
                        // match declaration type
                        match self.tokenizer.get_token() {
                            // requirement declaration
                            Ok(Some(Token::Keyword(KeywordName::Requirements))) => {
                                // TODO: handle errors
                                let requirements = self.parse_requirements()?;
                                for requirement in requirements {
                                    syntax_tree.add_requirement(requirement);
                                }
                            },
                            // objects declaration
                            Ok(Some(Token::Keyword(KeywordName::Objects))) => {
                                let objects = self.parse_list()?;
                                for object in objects.arguments {
                                    match object.var_type {
                                        Some(t) => {
                                            syntax_tree.add_typed_object(object.name, t);
                                        }
                                        None => {
                                            syntax_tree.add_object(object.name);
                                        }
                                    }
                                }
                            },
                            // initial task network declaration
                            Ok(Some(Token::Keyword(KeywordName::HTN))) => {
                                let init_tn = self.parse_initial_tn()?;
                                syntax_tree.add_init_tn(init_tn);
                            }
                            _ => todo!(),
                        }
                    } else {
                        // TODO: better error handling
                        panic!("expected '('")
                    }
                    Ok(syntax_tree)
                },
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
                        if let Ok(Some(Token::Identifier(domain_name))) = next_token {
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

    fn parse_requirements(&self) -> Result<Vec<RequirementType>, SyntacticError> {
        let mut requirements = vec![];
        let mut finished = false;
        while !finished {
            match self.tokenizer.get_token() {
                Ok(Some(Token::Requirement(req))) => {
                    requirements.push(req);
                }
                Ok(Some(Token::Punctuator(PunctuationType::RParentheses))) => {
                    finished = true;
                }
                _ => {
                    // TODO: better error handling
                    panic!("not a valid requirement")
                }
            }
        }
        Ok(requirements)
    }
}
