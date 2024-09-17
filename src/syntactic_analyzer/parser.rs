use self::definition_types::ProblemDefinition;

use super::*;

pub struct Parser<'a> {
    pub tokenizer: LexicalAnalyzer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: LexicalAnalyzer<'a>) -> Parser<'a> {
        Parser { tokenizer }
    }
    pub fn parse(&'a self) -> Result<SyntaxTree<'a>, ParsingError<'a>> {
        let mut syntax_tree = SyntaxTree::new();
        // match opening '('
        match self.tokenizer.get_token()? {
            TokenType::Punctuator(PunctuationType::LParentheses) => {
                // Determine file type
                match self.parse_document_type()? {
                    // Domain Definition
                    DefinitionType::Domain(domain_name) => {
                        loop {
                            match self.tokenizer.get_token()? {
                                TokenType::Punctuator(PunctuationType::LParentheses) => {
                                    match self.tokenizer.get_token()? {
                                        // predicate definition
                                        TokenType::Keyword(KeywordName::Predicates) => {
                                            let predicates = self.parse_predicates()?;
                                            for predicate in predicates {
                                                syntax_tree.add_predicate(predicate);
                                            }
                                        }
                                        // compund task definition
                                        TokenType::Keyword(KeywordName::Task) => {
                                            let task = self.parse_task()?;
                                            match self.tokenizer.get_token()? {
                                                TokenType::Punctuator(
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
                                        TokenType::Keyword(KeywordName::Method) => {
                                            let method = self.parse_method()?;
                                            syntax_tree.add_method(method);
                                        }
                                        // action definition
                                        TokenType::Keyword(KeywordName::Action) => {
                                            let action = self.parse_action()?;
                                            syntax_tree.add_action(action);
                                        }
                                        // requirement declaration
                                        TokenType::Keyword(KeywordName::Requirements) => {
                                            let requirements = self.parse_requirements()?;
                                            for requirement in requirements {
                                                syntax_tree.add_requirement(requirement);
                                            }
                                        }
                                        // type hierarchy declaration
                                        TokenType::Keyword(KeywordName::Types) => {
                                            let var_types = self.parse_args()?;
                                            for var_type in var_types {
                                                syntax_tree.add_var_type(var_type);
                                            }
                                        }
                                        // constants declaration
                                        TokenType::Keyword(KeywordName::Constants) => {
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
                                TokenType::Punctuator(PunctuationType::RParentheses) => {
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
                                TokenType::Punctuator(PunctuationType::LParentheses) => {
                                    // match declaration type
                                    match self.tokenizer.get_token()? {
                                        // requirement declaration
                                        TokenType::Keyword(KeywordName::Requirements) => {
                                            let requirements = self.parse_requirements()?;
                                            for requirement in requirements {
                                                syntax_tree.add_requirement(requirement);
                                            }
                                        }
                                        // objects declaration
                                        TokenType::Keyword(KeywordName::Objects) => {
                                            let objects = self.parse_args()?;
                                            for object in objects {
                                                match object.symbol_type {
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
                                        TokenType::Keyword(KeywordName::HTN) => {
                                            let init_tn = self.parse_initial_tn()?;
                                            syntax_tree.add_init_tn(init_tn);
                                        }
                                        // goal state (optional)
                                        TokenType::Keyword(KeywordName::Goal) => {
                                            let goal = self.parse_formula()?;
                                            syntax_tree.add_goal(goal)
                                        }
                                        // initial state
                                        TokenType::Keyword(KeywordName::Init) => {
                                            let init_state = self.parse_predicates()?;
                                            syntax_tree.add_init_state(init_state)
                                        }
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
                                TokenType::EOF | TokenType::Punctuator(PunctuationType::RParentheses) => {
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
            TokenType::Keyword(KeywordName::Define) => {
                // match '(' after keyword 'define
                match self.tokenizer.get_token()? {
                    TokenType::Punctuator(PunctuationType::LParentheses) => {
                        // match either 'domain' or 'problem'
                        match self.tokenizer.get_token()? {
                            TokenType::Keyword(KeywordName::Domain) => {
                                return self.parse_domain_header();
                            }
                            TokenType::Keyword(KeywordName::Problem) => {
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
            TokenType::Identifier(domain_name) => {
                // match closing paranthesis
                match self.tokenizer.get_token()? {
                    TokenType::Punctuator(PunctuationType::RParentheses) => {
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
            TokenType::Identifier(problem_name) => {
                // match closing paranthesis
                match self.tokenizer.get_token()? {
                    TokenType::Punctuator(PunctuationType::RParentheses) => {
                        // match '(' for domain name
                        match self.tokenizer.get_token()? {
                            TokenType::Punctuator(PunctuationType::LParentheses) => {
                                match self.tokenizer.get_token()? {
                                    TokenType::Keyword(KeywordName::Domain) => {
                                        match self.tokenizer.get_token()? {
                                            TokenType::Identifier(domain_name) => {
                                                match self.tokenizer.get_token()? {
                                                    TokenType::Punctuator(
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
                TokenType::Requirement(req) => {
                    requirements.push(req);
                }
                TokenType::Punctuator(PunctuationType::RParentheses) => {
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
