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
        if let Ok(Token::Punctuator(PunctuationType::LParentheses)) =
            self.tokenizer.get_token()
        {
            // Determine file type
            match self.parse_document_type()? {
                // Domain Definition
                DefinitionType::Domain(_) => {
                    loop {
                        match self.tokenizer.get_token() {
                            Ok(Token::Punctuator(PunctuationType::LParentheses)) => {
                                match self.tokenizer.get_token() {
                                    // predicate definition
                                    Ok(Token::Keyword(KeywordName::Predicates)) => {
                                        let predicates = self.parse_predicates()?;
                                        for predicate in predicates {
                                            syntax_tree.add_predicate(predicate);
                                        }
                                    }
                                    // compund task definition
                                    Ok(Token::Keyword(KeywordName::Task)) => {
                                        let task: Task = self.parse_task()?;
                                        if let Ok(Token::Punctuator(
                                            PunctuationType::RParentheses,
                                        )) = self.tokenizer.get_token()
                                        {
                                            syntax_tree.add_compound_task(task);
                                        } else {
                                            panic!("task block not closed");
                                        }
                                    }
                                    // method definition
                                    Ok(Token::Keyword(KeywordName::Method)) => {
                                        let method = self.parse_method()?;
                                        syntax_tree.add_method(method);
                                    }
                                    // action definition
                                    Ok(Token::Keyword(KeywordName::Action)) => {
                                        let action = self.parse_action()?;
                                        syntax_tree.add_action(action);
                                    }
                                    // requirement declaration
                                    Ok(Token::Keyword(KeywordName::Requirements)) => {
                                        let requirements = self.parse_requirements()?;
                                        for requirement in requirements {
                                            syntax_tree.add_requirement(requirement);
                                        }
                                    }
                                    Ok(Token::Keyword(KeywordName::Types)) => {
                                        let var_types = self.parse_args()?;
                                        for var_type in var_types {
                                            syntax_tree.add_var_type(var_type);
                                        }
                                    }
                                    Ok(Token::Keyword(KeywordName::Constants)) => {
                                        let constants = self.parse_args()?;
                                        for constant in constants {
                                            syntax_tree.add_constant(constant);
                                        }
                                    }
                                    err => {
                                        // TODO: better error handling
                                        panic!("expected a keyword {:?}", err)
                                    }
                                }
                            }
                            Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                                break;
                            }
                            _ => {
                                panic!("undefined");
                            }
                        }
                    }
                    return Ok(syntax_tree);
                }
                // Problem Definition
                DefinitionType::Problem(_) => {
                    loop {
                        match self.tokenizer.get_token() {
                            Ok(Token::Punctuator(PunctuationType::LParentheses)) => {
                                // match declaration type
                                match self.tokenizer.get_token() {
                                    // requirement declaration
                                    Ok(Token::Keyword(KeywordName::Requirements)) => {
                                        let requirements = self.parse_requirements()?;
                                        for requirement in requirements {
                                            syntax_tree.add_requirement(requirement);
                                        }
                                    }
                                    // objects declaration
                                    Ok(Token::Keyword(KeywordName::Objects)) => {
                                        let objects = self.parse_args()?;
                                        for object in objects {
                                            match object.var_type {
                                                Some(t) => {
                                                    syntax_tree.add_typed_object(object.name, t);
                                                }
                                                None => {
                                                    syntax_tree.add_object(object.name);
                                                }
                                            }
                                        }
                                    }
                                    // initial task network declaration
                                    Ok(Token::Keyword(KeywordName::HTN)) => {
                                        let init_tn = self.parse_initial_tn()?;
                                        syntax_tree.add_init_tn(init_tn);
                                    }
                                    _ => todo!(),
                                }
                            }
                            Ok(Token::EOF) | Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
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
        } else {
            // TODO: improve error handling
            panic!("files should start with '('")
        }
    }

    fn parse_document_type(&self) -> Result<DefinitionType, ParsingError> {
        // match keyword 'define'
        if let Ok(Token::Keyword(KeywordName::Define)) = self.tokenizer.get_token() {
            // match '(' after keyword 'define
            if let Ok(Token::Punctuator(PunctuationType::LParentheses)) =
                self.tokenizer.get_token()
            {
                // match either 'domain' or 'problem'
                match self.tokenizer.get_token() {
                    Ok(Token::Keyword(KeywordName::Domain)) => {
                        // match domain name
                        let next_token = self.tokenizer.get_token();
                        if let Ok(Token::Identifier(domain_name)) = next_token {
                            // match closing paranthesis
                            if let Ok(Token::Punctuator(PunctuationType::RParentheses)) =
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
                    Ok(Token::Keyword(KeywordName::Problem)) => {
                        // match problem name
                        if let Ok(Token::Identifier(problem_name)) =
                            self.tokenizer.get_token()
                        {
                            // match closing paranthesis
                            if let Ok(Token::Punctuator(PunctuationType::RParentheses)) =
                                self.tokenizer.get_token()
                            {
                                // match '(' for domain name
                                if let Ok(Token::Punctuator(PunctuationType::LParentheses)) =
                                    self.tokenizer.get_token()
                                {
                                    if let Ok(Token::Keyword(KeywordName::Domain)) =
                                        self.tokenizer.get_token()
                                    {
                                        if let Ok(Token::Identifier(domain_name)) =
                                            self.tokenizer.get_token()
                                        {
                                            if let Ok(Token::Punctuator(
                                                PunctuationType::RParentheses,
                                            )) = self.tokenizer.get_token()
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
                Ok(Token::Requirement(req)) => {
                    requirements.push(req);
                }
                Ok(Token::Punctuator(PunctuationType::RParentheses)) => {
                    finished = true;
                }
                _ => {
                    // TODO: better error handling
                    panic!("not a valid requirement")
                }
            }
        }
        return Ok(requirements);
        
    }
}
