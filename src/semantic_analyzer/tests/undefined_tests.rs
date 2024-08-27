use analyzer::verify_semantics;

use super::*;

#[test]
pub fn undefined_predicate_action_precondition_test() {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (hold ?a_1 ?a_2)
                (pred_2)
                (at a_1)
            )
            (:action a_1
             :parameters (p_1 p_2 p_3)
             :precondition (and (not (at p_1)) (pred_5))
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
                SemanticError::UndefinedPredicate(x) => {
                    assert_eq!(x, "pred_5")
                    // TODO: assert locality in future
                }
                token => {
                    panic!("{:?}", token)
                }
            }
        }
    }
}

#[test]
pub fn inconsistent_predicate_action_effect_test() {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (hold ?a_1 ?a_2)
                (pred_2)
                (at ?a_1)
            )
            (:action a_1
             :parameters (?p_1 ?p_2 ?p_3)
             :precondition (and (not (at ?p_1)) (hold ?p1 ?p2))
             :effect (and (not (hold ?p_2 ?p_3 p_2)) (at ?p_2))
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
                SemanticError::InconsistentPredicateArity(x) => {
                    assert_eq!(x, "hold")
                    // TODO: assert locality in future
                }
                token => {
                    panic!("{:?}", token)
                }
            }
        }
    }
}


#[test]
pub fn undefined_predicate_method_precondition_test() {
    let program = String::from(
        "(define (domain bal)
                (:predicates 
                    (hold ?a_1 ?a_2)
                    (pred_2)
                    (at ?a_1)
                )
                (:method m_1
                    :parameters (?p1 ?l1 ?l2 ?l3) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :precondition (oneof (and (not (hold ?p_2 ?p_3)) (at ?p_2)) (pred_5))
                    :subtasks (and
                        (pickup ?p1 ?l1)
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
                SemanticError::UndefinedPredicate(x) => {
                    assert_eq!(x, "pred_5")
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
pub fn undefined_subtask_test() {
    let program = String::from(
        "(define (domain bal)
                (:predicates 
                    (hold ?a_1 ?a_2)
                    (pred_2)
                    (at ?a_1)
                )
                (:task c_1
                    :parameters (?p_1 ?p_2 ?p_3)
                )
                (:task c_2
                    :parameters (?p_1)
                )
                (:method m_1
                    :parameters (?p1 ?l1 ?l2 ?l3) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :precondition (oneof (and (not (hold ?p_2 ?p_3)) (at ?p_2)) (pred_2))
                    :subtasks (and
                        (c_1 ?p1 ?l1 ?l2)
                        (c_2 ?p1)
                        (c_3)
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
                SemanticError::UndefinedSubtask(x) => {
                    assert_eq!(x, "c_3")
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
pub fn inconsistent_subtask_arity_test() {
    let program = String::from(
        "(define (domain bal)
                (:predicates 
                    (hold ?a_1 ?a_2)
                    (pred_2)
                    (at ?a_1)
                )
                (:task c_1
                    :parameters (?p_1 ?p_2 ?p_3)
                )
                (:task c_2
                    :parameters (?p_1)
                )
                (:method m_1
                    :parameters (?p1 ?l1 ?l2 ?l3) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :precondition (oneof (and (not (hold ?p_2 ?p_3)) (at ?p_2)) (pred_2))
                    :subtasks (and
                        (c_1 ?p1 ?l1 ?l2)
                        (c_2 ?p1 ?l3)
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
                SemanticError::InconsistentTaskArity(x) => {
                    assert_eq!(x, "c_2")
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
pub fn undefined_type_compound_task_test() {
    let program = String::from(
        "(define (domain bal)
                (:types t1)
                (:predicates 
                    (hold ?a_1 ?a_2)
                    (pred_2)
                    (at ?a_1)
                )
                (:task c_1
                    :parameters (?p_1 ?p_2 ?p_3 - t1 ?p4 - t5)
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
                SemanticError::UndefinedType(x) => {
                    assert_eq!(x, "t5")
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
pub fn undefined_type_predicate_test() {
    let program = String::from(
        "(define (domain bal)
                (:types t1)
                (:predicates 
                    (pred_2)
                    (at ?a_1)
                    (hold ?a_1 ?a_2 - t1 ?a_3 - t2)
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
                SemanticError::UndefinedType(x) => {
                    assert_eq!(x, "t2")
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}
