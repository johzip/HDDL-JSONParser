use serde_json::{json, Value};
use crate::lexical_analyzer::LexicalAnalyzer;
pub use crate::output::{ ParsingError, SemanticErrorType, SyntacticError, WarningType};
use crate::syntactic_analyzer;
use crate::syntactic_analyzer::{AbstractSyntaxTree, Formula, Predicate, Action, Symbol};

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

                            let primitive = self.actions_to_json(d.actions);

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
                                    "primitive_tasks": primitive,
                                    "compound_tasks": [
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
                            "compound_tasks": d.compound_tasks
                        }
                    }
                });
                    Ok(serde_json::to_string_pretty(&json).unwrap())
                }
            },
            _ => panic!("expected domain, found problem"),
        }
    }

    fn actions_to_json<'a>(&self, actions: Vec<Action<'a>>) -> Vec<Value> {
        actions
            .iter()
            .map(|action| {

                let parameters_json: Vec<_> = action.parameters.iter()
                    .map(|var| json!({
                    "name": var.name,
                    "type": var.symbol_type.unwrap_or("unknown")
                }))
                    .collect();

                let precondition_json = match &action.preconditions {
                    Some(formula) => self.tasks_call_to_json(formula),
                    None => vec![],
                };
                let effect_json = match &action.effects {
                    Some(formula) => self.tasks_call_to_json(formula),
                    None => vec![],
                };

                json!({
                "name": action.name,
                "parameters": parameters_json,
                "precondition": precondition_json,
                "effect": effect_json
            })
            })
            .collect()
    }

    fn init_to_json<'a>(&self, init_state: Vec<Predicate<'a>>) -> Vec<Value> {
        init_state
            .iter()
            .map(|pred| {
                let parameters_json: Vec<_> = pred.variables.iter()
                    .map(|var| json!(var.name))
                    .collect();
                json!({
                "name": pred.name,
                "type": "predicate",
                "parameters": parameters_json
            })
            })
            .collect()
    }

    fn tasks_call_to_json<'a>(&self, formula: &Formula<'a>) -> Vec<serde_json::Value> {
        match formula {
            Formula::Empty => vec![],
            Formula::Atom(pred) => self.predicate_to_json(pred),
            Formula::And(terms) => {
                if terms.len() == 2 {
                    vec!{json!({
                        "type": "and",
                        "left": self.tasks_call_to_json(&terms[0]),
                        "right": self.tasks_call_to_json(&terms[1])
                    })}
                } else {
                    vec![json!({
                        "type": "and",
                        "expression": terms.iter().flat_map(|t| self.tasks_call_to_json(t)).collect::<Vec<_>>()
                    })]
                }
            },
            Formula::Or(terms) =>  vec!{json!({
                "type": "or",
                "left": self.tasks_call_to_json(&terms[0]),
                "right": self.tasks_call_to_json(&terms[1])
            })},
            Formula::Not(term) =>  vec!{json!({
                "type": "not",
                "expression": self.tasks_call_to_json(term.as_ref())
            })},
           Formula::Xor(terms) =>  vec!{json!({
                "type": "Xor",
                "left": self.tasks_call_to_json(&terms[0]),
                "right": self.tasks_call_to_json(&terms[1])
            })},
            Formula::Imply(lhs, rhs) => {
                let mut result = Vec::new();
                result.extend(lhs.iter().flat_map(|term| self.tasks_call_to_json(term.as_ref())));
                result.extend(rhs.iter().flat_map(|term| self.tasks_call_to_json(term.as_ref())));
                result
            }
            Formula::Exists(_, term) | Formula::ForAll(_, term) => self.tasks_call_to_json(term.as_ref()),
            Formula::Equals(left, right) => {
                vec!{json!({
                "type": "equals",
                "left": left,
                "right": right
            })}
            }
        }
    }


    fn predicate_to_json(&self, pred: &Predicate) -> Vec<Value> {
        let parameters_json = self.parameters_to_json(&pred.variables);
        vec!{json!({
                "name": pred.name,
                "type": "predicate",
                "parameters": parameters_json
            })}
    }

    fn parameters_to_json<'a>(&self, parameters: &Vec<Symbol<'a>>) -> Vec<Value> {
        parameters
            .iter()
            .map(|var| json!({"name": var.name, "type": var.symbol_type.unwrap_or("unknown")}))
            .collect()
    }
}

//Problem:
//TODO: Atom: replace with hpdl style -Done
//TODO: remove lineNumbers -Done
//TODO: parameter always has type unknown so maby just remove it - Done
//TODO: parameter instead of variable in goal - DONE

//Domain:
//TODO: primitive_tasks in effects, the Not and Or are missing - DONE
//TODO: primitive_tasks effects can be add and del predicate or task call
//TODO: primitive_tasks remove lineNumbers - DONE
//TODO: primitive_tasks name only string - DONE
//TODO: primitive_tasks parameters instead of params - DONE
//TODO: primitive_tasks parameters change symbol_type to type - DONE
//TODO: primitive_tasks check precondition (null might be bug) - DONE

//TODO: compundtask and primitive task might be switched up (methods are compund tasks
//TODO: split up tn into ordering and subtasks
//TODO: what is tn in method? and what are its contents
//TODO: task and task term merge into taskcall (hpdl methods)

//TODO: actions are missing intierly
//TODO: requirements -> Problem -> Domain
//Questions:
// what is de difference between task and subtasks inside a method?