pub use crate::language_server::RequestHandler;

use std::collections::HashMap;

use serde_json::{json, Value};
use crate::lexical_analyzer::LexicalAnalyzer;
pub use crate::output::{LexicalErrorType, ParsingError, SemanticErrorType, SyntacticError, WarningType};
use crate::syntactic_analyzer;
use crate::syntactic_analyzer::{AbstractSyntaxTree, Formula, Predicate};

pub struct HDDLJsonParser;

impl HDDLJsonParser {
    pub fn to_json(&self, domain: &Vec<u8>, problem: Option<&Vec<u8>>) -> Result<String, ParsingError> {
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
                            let goal = match &p.goal {
                                Some(formula) => self.tasks_call_to_json(formula),
                                None => vec![],
                            };

                            let init = self.init_to_json(p.init_state);

                            let json = serde_json::json!({
                            d.name.clone(): {
                                "requirements": [
                                    d.requirements
                                ],
                                "problem": {
                                    "goal": {
                                        "tasks": goal
                                    },
                                    "init": init
                                },
                                "domain": {
                                    "name": d.name,
                                    "primitive_tasks": d.actions,
                                    "compund_tasks": [
                                            d.compound_tasks,
                                            d.methods
                                        ]
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

    fn init_to_json<'a>(&self, init_state: Vec<Predicate<'a>>) -> Vec<Value> {
        init_state
            .iter()
            .flat_map(|pred| Self::predicate_to_json(pred))
            .collect()
    }

    fn tasks_call_to_json<'a>(&self, formula: &Formula<'a>) -> Vec<serde_json::Value> {
        match formula {
            Formula::Empty => vec![],
            Formula::Atom(pred) => Self::predicate_to_json(pred),
            Formula::Not(term) => self.tasks_call_to_json(term.as_ref()),
            Formula::And(terms) | Formula::Or(terms) | Formula::Xor(terms) => {
                terms.iter().flat_map(|term| self.tasks_call_to_json(term.as_ref())).collect()
            }
            Formula::Imply(lhs, rhs) => {
                let mut result = Vec::new();
                result.extend(lhs.iter().flat_map(|term| self.tasks_call_to_json(term.as_ref())));
                result.extend(rhs.iter().flat_map(|term| self.tasks_call_to_json(term.as_ref())));
                result
            }
            Formula::Exists(_, term) | Formula::ForAll(_, term) => self.tasks_call_to_json(term.as_ref()),
            Formula::Equals(left, right) => {
                vec![json!({
                "type": "equals",
                "left": left,
                "right": right
            })]
            }
        }
    }

    fn predicate_to_json(pred: &Predicate) -> Vec<Value> {
        let parameters_json: Vec<_> = pred.variables.iter()
            .map(|var| json!(var.name))
            .collect();
        vec![json!({
                "name": pred.name,
                "type": "predicate",
                "parameters": parameters_json
            })]
    }
}

//Problem:
//TODO: Atom: replace with hpdl style
//TODO: remove lineNumbers
//TODO: parameter always has type unknown so maby just remove it
//TODO: parameter instead of variable in goal - DONE

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