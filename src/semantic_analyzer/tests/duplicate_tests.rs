use analyzer::verify_semantics;

use super::*;

#[test]
pub fn objects_duplicate_test() {
    let program = String::from(
        "(define (problem p1) (domain bal)
                            (:objects a b c - d b - f t)
                          )",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(program);
    match verify_semantics(Parser::new(&lexer).parse().as_ref().unwrap()) {
        Ok(_) => {
            panic!("errors are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::DuplicateObjectDeclaration(x) => {
                    assert_eq!(x, "b");
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}

#[test]
pub fn requirements_duplicate_test() {
    let program = String::from(
        "(define (problem p1) (domain bal)
            (:requirements :hierarchy :method-preconditions :hierarchy :negative-preconditions)

         ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(program);
    match verify_semantics(Parser::new(&lexer).parse().as_ref().unwrap()) {
        Ok(_) => {
            panic!("errors are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::DuplicateRequirementDeclaration(x) => {
                    assert!(matches!(x, RequirementType::Hierarchy))
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}

#[test]
pub fn predicates_duplicate_test() {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (pred_1 ?a_1 ?a_2 - t_1 ?a_3 - t_2)
                (pred_2)
                (pred_1 a_1 a_2)
            )
         ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(program);
    match verify_semantics(Parser::new(&lexer).parse().as_ref().unwrap()) {
        Ok(_) => {
            panic!("errors are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::DuplicatePredicateDeclaration(x) => {
                    assert_eq!(x, "pred_1")
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}

#[test]
pub fn action_duplicate_test() {
    let program = String::from(
        "(define (domain bal)
            (:action a_1
             :parameters (p_1 p_2 - t1 p_3 - t2)
             :precondition (not (at p_1))
             :effect (and (not (hold p_2 p_3)) (at p_2))
            )
            (:action a_2
             :parameters (p_1 p_2 - t1)
             :precondition (not (at p_1))
             :effect (and (not (at p_2)))
            )
            (:action a_1
             :parameters (p_1 p_2 - t1 p_3 p4 p5 - t2)
             :precondition (not (at p_1))
             :effect (and (not (hold p_2 p_3)) (at p_2))
            )
         ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(program);
    match verify_semantics(Parser::new(&lexer).parse().as_ref().unwrap()) {
        Ok(_) => {
            panic!("errors are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::DuplicateActionDeclaration(x) => {
                    assert_eq!(x, "a_1")
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}

#[test]
pub fn compound_task_duplicate_test() {
    let program = String::from(
        "(define (domain bal)
                (:task c_1
                 :parameters (p_1 p_2 - t1 p_3 - t2)
                )
                (:task c_2
                 :parameters (p_1 - t1)
                )
                (:task c_1
                 :parameters (p_1 p_2 - t1)
                )
             ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(program);
    match verify_semantics(Parser::new(&lexer).parse().as_ref().unwrap()) {
        Ok(_) => {
            panic!("errors are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::DuplicateCompoundTaskDeclaration(x) => {
                    assert_eq!(x, "c_1")
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}

#[test]
pub fn method_duplicate_test() {
    let program = String::from(
        "(define (domain bal)
                (:method m_1
                    :parameters (?p1 - p ?l1 ?l2 ?l3 - loc) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :subtasks (and
                        (pickup ?p1 ?l1)
                        (deliver_abs ?p1 ?l2 ?l3)
                    )
                )
                (:method m_2
                    :parameters (?p1 - p ?l1 ?l2 - loc) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :subtasks (and
                        (pickup ?p1 ?l1)
                    )
                )
                (:method m_1
                    :parameters (?p1 - p ?l1 ?l2 ?l3 - loc) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :subtasks (and
                        (deliver_abs ?p1 ?l2 ?l3)
                    )
                )
             ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(program);
    match verify_semantics(Parser::new(&lexer).parse().as_ref().unwrap()) {
        Ok(_) => {
            panic!("errors are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::DuplicateMethodDeclaration(x) => {
                    assert_eq!(x, "m_1")
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}
