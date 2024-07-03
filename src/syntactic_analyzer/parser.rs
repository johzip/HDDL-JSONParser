use crate::parsing_errors::ParsingError;

use self::definition_types::ProblemDefinition;

use super::*;

pub struct Parser<'a> {
    pub tokenizer: &'a LexicalAnalyzer,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a LexicalAnalyzer) -> Parser<'a> {
        Parser { tokenizer }
    }
    pub fn parse(&'a self) -> Result<SyntaxTree<'a>, ParsingError<'a>> {
        let mut syntax_tree = SyntaxTree::new();
        // match opening '('
        match self.tokenizer.get_token()? {
            Token::Punctuator(PunctuationType::LParentheses) => {
                // Determine file type
                match self.parse_document_type()? {
                    // Domain Definition
                    DefinitionType::Domain(domain_name) => {
                        loop {
                            match self.tokenizer.get_token()? {
                                Token::Punctuator(PunctuationType::LParentheses) => {
                                    match self.tokenizer.get_token()? {
                                        // predicate definition
                                        Token::Keyword(KeywordName::Predicates) => {
                                            let predicates = self.parse_predicates()?;
                                            for predicate in predicates {
                                                syntax_tree.add_predicate(predicate);
                                            }
                                        }
                                        // compund task definition
                                        Token::Keyword(KeywordName::Task) => {
                                            let task = self.parse_task()?;
                                            match self.tokenizer.get_token()? {
                                                Token::Punctuator(
                                                    PunctuationType::RParentheses,
                                                ) => {
                                                    syntax_tree.add_compound_task(task);
                                                }
                                                token => {
                                                    let error = SyntacticError {
                                                        expected: format!(
                                                            "')' after definition of {}",
                                                            task.name
                                                        )
                                                        .to_string(),
                                                        found: token,
                                                        line_number: self
                                                            .tokenizer
                                                            .get_line_number(),
                                                    };
                                                    return Err(ParsingError::Syntactic(error));
                                                }
                                            }
                                        }
                                        // method definition
                                        Token::Keyword(KeywordName::Method) => {
                                            let method = self.parse_method()?;
                                            syntax_tree.add_method(method);
                                        }
                                        // action definition
                                        Token::Keyword(KeywordName::Action) => {
                                            let action = self.parse_action()?;
                                            syntax_tree.add_action(action);
                                        }
                                        // requirement declaration
                                        Token::Keyword(KeywordName::Requirements) => {
                                            let requirements = self.parse_requirements()?;
                                            for requirement in requirements {
                                                syntax_tree.add_requirement(requirement);
                                            }
                                        }
                                        // type hierarchy declaration
                                        Token::Keyword(KeywordName::Types) => {
                                            let var_types = self.parse_args()?;
                                            for var_type in var_types {
                                                syntax_tree.add_var_type(var_type);
                                            }
                                        }
                                        // constants declaration
                                        Token::Keyword(KeywordName::Constants) => {
                                            let constants = self.parse_args()?;
                                            for constant in constants {
                                                syntax_tree.add_constant(constant);
                                            }
                                        }
                                        token => {
                                            let error = SyntacticError {
                                                expected: "a keyword".to_string(),
                                                found: token,
                                                line_number: self.tokenizer.get_line_number(),
                                            };
                                            return Err(ParsingError::Syntactic(error));
                                        }
                                    }
                                }
                                Token::Punctuator(PunctuationType::RParentheses) => {
                                    break;
                                }
                                token => {
                                    let error = SyntacticError {
                                        expected: format!("either ')' to close the definition of {}, or '(' to start defining new components", domain_name),
                                        found: token,
                                        line_number: self.tokenizer.get_line_number(),
                                    };
                                    return Err(ParsingError::Syntactic(error));
                                }
                            }
                        }
                        return Ok(syntax_tree);
                    }
                    // Problem Definition
                    DefinitionType::Problem(problem_name) => {
                        loop {
                            match self.tokenizer.get_token()? {
                                Token::Punctuator(PunctuationType::LParentheses) => {
                                    // match declaration type
                                    match self.tokenizer.get_token()? {
                                        // requirement declaration
                                        Token::Keyword(KeywordName::Requirements) => {
                                            let requirements = self.parse_requirements()?;
                                            for requirement in requirements {
                                                syntax_tree.add_requirement(requirement);
                                            }
                                        }
                                        // objects declaration
                                        Token::Keyword(KeywordName::Objects) => {
                                            let objects = self.parse_args()?;
                                            for object in objects {
                                                match object.var_type {
                                                    Some(t) => {
                                                        syntax_tree
                                                            .add_typed_object(object.name, t);
                                                    }
                                                    None => {
                                                        syntax_tree.add_object(object.name);
                                                    }
                                                }
                                            }
                                        }
                                        // initial task network declaration
                                        Token::Keyword(KeywordName::HTN) => {
                                            let init_tn = self.parse_initial_tn()?;
                                            syntax_tree.add_init_tn(init_tn);
                                        }
                                        // goal state (optional)
                                        Token::Keyword(KeywordName::Goal) => {
                                            let goal = self.parse_formula()?;
                                            syntax_tree.add_goal(goal)
                                        }
                                        // initial state
                                        // TODO:
                                        Token::Keyword(KeywordName::Init) => {}
                                        token => {
                                            let error = SyntacticError {
                                                expected: "a keyword for block definition"
                                                    .to_string(),
                                                found: token,
                                                line_number: self.tokenizer.get_line_number(),
                                            };
                                            return Err(ParsingError::Syntactic(error));
                                        }
                                    }
                                }
                                Token::EOF | Token::Punctuator(PunctuationType::RParentheses) => {
                                    break;
                                }
                                err => {
                                    panic!("unexpected token {:?}", err)
                                }
                            }
                        }
                        Ok(syntax_tree)
                    }
                }
            }
            token => {
                let error = SyntacticError {
                    expected: "start of the file with '('".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }
    }

    fn parse_document_type(&self) -> Result<DefinitionType, ParsingError> {
        // match keyword 'define'
        match self.tokenizer.get_token()? {
            Token::Keyword(KeywordName::Define) => {
                // match '(' after keyword 'define
                match self.tokenizer.get_token()? {
                    Token::Punctuator(PunctuationType::LParentheses) => {
                        // match either 'domain' or 'problem'
                        match self.tokenizer.get_token()? {
                            Token::Keyword(KeywordName::Domain) => {
                                return self.parse_domain_header();
                            }
                            Token::Keyword(KeywordName::Problem) => {
                                return self.parse_problem_header();
                            }
                            token => {
                                let error = SyntacticError {
                                    expected: "either keyword 'domain' or 'problem'".to_string(),
                                    found: token,
                                    line_number: self.tokenizer.get_line_number(),
                                };
                                return Err(ParsingError::Syntactic(error));
                            }
                        }
                    }
                    token => {
                        let error = SyntacticError {
                            expected: "'(' after keyword 'define'".to_string(),
                            found: token,
                            line_number: self.tokenizer.get_line_number(),
                        };
                        return Err(ParsingError::Syntactic(error));
                    }
                }
            }
            token => {
                let error = SyntacticError {
                    expected: "keyword 'define'".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }
    }

    fn parse_domain_header(&self) -> Result<DefinitionType, ParsingError> {
        match self.tokenizer.get_token()? {
            Token::Identifier(domain_name) => {
                // match closing paranthesis
                match self.tokenizer.get_token()? {
                    Token::Punctuator(PunctuationType::RParentheses) => {
                        return Ok(DefinitionType::Domain(domain_name));
                    }
                    token => {
                        let error = SyntacticError {
                            expected: "')'".to_string(),
                            found: token,
                            line_number: self.tokenizer.get_line_number(),
                        };
                        return Err(ParsingError::Syntactic(error));
                    }
                }
            }
            token => {
                let error = SyntacticError {
                    expected: "domain name".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }
    }

    fn parse_problem_header(&self) -> Result<DefinitionType, ParsingError> {
        // match problem name
        match self.tokenizer.get_token()? {
            Token::Identifier(problem_name) => {
                // match closing paranthesis
                match self.tokenizer.get_token()? {
                    Token::Punctuator(PunctuationType::RParentheses) => {
                        // match '(' for domain name
                        match self.tokenizer.get_token()? {
                            Token::Punctuator(PunctuationType::LParentheses) => {
                                match self.tokenizer.get_token()? {
                                    Token::Keyword(KeywordName::Domain) => {
                                        match self.tokenizer.get_token()? {
                                            Token::Identifier(domain_name) => {
                                                match self.tokenizer.get_token()? {
                                                    Token::Punctuator(
                                                        PunctuationType::RParentheses,
                                                    ) => {
                                                        return Ok(DefinitionType::Problem(
                                                            ProblemDefinition {
                                                                domain_name: domain_name,
                                                                problem_name: problem_name,
                                                            },
                                                        ));
                                                    }
                                                    token => {
                                                        let error = SyntacticError {
                                                            expected: format!("the block of the definition of problem '{}' is not closed with ')'", problem_name),
                                                            found: token,
                                                            line_number: self.tokenizer.get_line_number(),
                                                        };
                                                        return Err(ParsingError::Syntactic(error));
                                                    }
                                                }
                                            }
                                            token => {
                                                let error = SyntacticError {
                                                    expected: "domain name".to_string(),
                                                    found: token,
                                                    line_number: self.tokenizer.get_line_number(),
                                                };
                                                return Err(ParsingError::Syntactic(error));
                                            }
                                        }
                                    }
                                    token => {
                                        let error = SyntacticError {
                                            expected: "keyword 'domain'".to_string(),
                                            found: token,
                                            line_number: self.tokenizer.get_line_number(),
                                        };
                                        return Err(ParsingError::Syntactic(error));
                                    }
                                }
                            }
                            token => {
                                let error = SyntacticError {
                                    expected: "'('".to_string(),
                                    found: token,
                                    line_number: self.tokenizer.get_line_number(),
                                };
                                return Err(ParsingError::Syntactic(error));
                            }
                        }
                    }
                    token => {
                        let error = SyntacticError {
                            expected: "')'".to_string(),
                            found: token,
                            line_number: self.tokenizer.get_line_number(),
                        };
                        return Err(ParsingError::Syntactic(error));
                    }
                }
            }
            token => {
                let error = SyntacticError {
                    expected: "problem name".to_string(),
                    found: token,
                    line_number: self.tokenizer.get_line_number(),
                };
                return Err(ParsingError::Syntactic(error));
            }
        }
    }

    fn parse_requirements(&self) -> Result<Vec<RequirementType>, ParsingError> {
        let mut requirements = vec![];
        let mut finished = false;
        while !finished {
            match self.tokenizer.get_token()? {
                Token::Requirement(req) => {
                    requirements.push(req);
                }
                Token::Punctuator(PunctuationType::RParentheses) => {
                    finished = true;
                }
                token => {
                    let error = SyntacticError {
                        expected: "either a requirement or a ')'".to_string(),
                        found: token,
                        line_number: self.tokenizer.get_line_number(),
                    };
                    return Err(ParsingError::Syntactic(error));
                }
            }
        }
        return Ok(requirements);
    }
}
