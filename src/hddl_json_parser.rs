pub use crate::language_server::RequestHandler;

use std::collections::HashMap;

use crate::lexical_analyzer::LexicalAnalyzer;
pub use crate::output::{LexicalErrorType, ParsingError, SemanticErrorType, SyntacticError, WarningType};
use crate::syntactic_analyzer;
use crate::syntactic_analyzer::AbstractSyntaxTree;

pub struct HDDLJsonParser;

impl HDDLJsonParser {
    pub fn to_json(domain: &Vec<u8>, problem: Option<&Vec<u8>>) -> Result<String, ParsingError> {
        let lexer = LexicalAnalyzer::new(&domain);
        let domain_parser = syntactic_analyzer::Parser::new(lexer);
        let domain_ast = domain_parser.parse()?;
        match domain_ast {
            AbstractSyntaxTree::Domain(d) => match problem {
                Some(p) => {
                    let lexer = LexicalAnalyzer::new(p);
                    let problem_parser = syntactic_analyzer::Parser::new(lexer);
                    let problem_ast = problem_parser.parse()?;
                    match problem_ast {
                        AbstractSyntaxTree::Problem(p) => {

                            let json = serde_json::json!({
                            d.name.clone(): {
                                "requirements": [
                                    d.requirements
                                ],
                                "problem": {
                                    "goal": {
                                        "tasks": p.goal // Anpassen!
                                    },
                                    "init": p.init_state // Anpassen!
                                },
                                "domain": {
                                    "name": d.name,
                                    "primitive_tasks": d.methods, // Anpassen!
                                    "compund_tasks": d.compound_tasks // Anpassen!
                                }
                            }
                        });
                            Ok(serde_json::to_string_pretty(&json).unwrap())
                        }
                        _ => panic!("expected problem, found domain"),
                    }
                }
                None => {
                    // only Domain, Problem is missing
                    let json = serde_json::json!({
                    d.name.clone(): {
                        "requirements": [
                            "strips",
                            "typing"
                        ],
                        "domain": {
                            "name": d.name,
                            "primitive_tasks": d.methods,
                            "compund_tasks": d.compound_tasks
                        }
                    }
                });
                    Ok(serde_json::to_string_pretty(&json).unwrap())
                }
            },
            _ => panic!("expected domain, found problem"),
        }
    }
}

//Problem:
//TODO: Atom: replace with hpdl style
//TODO: remove lineNumbers
//TODO: parameter instead of variable in goal

//Domain:
//TODO: primitive_tasks name only string
//TODO: primitive_tasks parameters instead of params
//TODO: primitive_tasks parameters change symbol_type to type
//TODO: primitive_tasks check precondition (null might be bug)
//TODO: compundtask and primitive task might be switched up (methods are compund tasks
//TODO: split up tn into ordering and subtasks
//TODO: what is tn in method? and what are its contents
//TODO: task and task term merge into taskcall (hpdl methods)

//TODO: actions are missing intierly
//TODO: requirements -> Problem -> Domain
//Questions:
// what is de difference between task and subtasks inside a method?