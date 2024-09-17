use super::*;

#[test]
pub fn basic_type_checking_test () {
    let program = String::from(
        "(define (domain bal)
            (:types
            t1 t2 - t3
            t4 t5 - t6
            t3 t6 - t7
            )
            (:predicates 
                (at ?l - t1)
            )
            (:action test1
            :parameters(?l1 - t2)
            :precondition (at ?l1)
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    let semantic_parser = SemanticAnalyzer::new(&ast);
    match semantic_parser.verify_ast() {
        Ok(_) => {
            panic!("errors are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::InconsistentPredicateArgType(t_err) => {
                    assert_eq!(t_err.var_name, "l1");
                    assert_eq!(t_err.found.unwrap(), "t2");
                    assert_eq!(t_err.expected.unwrap(), "t1");
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
pub fn effect_type_checking_test () {
    let program = String::from(
        "(define (domain bal)
            (:types
            t1 t2 - t3
            t4 t5 - t6
            t3 t6 - t7
            )
            (:predicates 
                (at ?l - t1)
            )
            (:action test1
            :parameters(?l1 ?l2 - t1)
            :precondition (at ?l2)
            :effect (not(at ?l1))
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    let semantic_parser = SemanticAnalyzer::new(&ast);
    match semantic_parser.verify_ast() {
        Ok(_) => {        }
        Err(error) => {
            panic!("{:?}", error)
        }
    }
}

#[test]
pub fn method_prec_type_checking_test () {
    let program = String::from(
        "(define (domain bal)
            (:types
            t1 t2 - t3
            t4 t5 - t6
            t3 t6 - t7
            )
            (:predicates 
                (at ?l - t7)
            )
            (:task abs :parameters(?a))
            (:action test1
            :parameters(?l1 ?l2 - t5)
            :precondition (at ?l2)
            :effect (not(at ?l1))
            )
            (:method m1
                :parameters(?l2 - t1 ?l1 - t5)
                :task (abs ?l)
                :precondition (at ?l2)
                :tasks (test1 ?l1 ?l1)
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    let semantic_parser = SemanticAnalyzer::new(&ast);
    match semantic_parser.verify_ast() {
        Ok(_) => {        }
        Err(error) => {
            panic!("{:?}", error)
        }
    }
}

#[test]
pub fn method_subtask_checking_test () {
    let program = String::from(
        "(define (domain bal)
            (:types
            t1 t2 - t3
            t4 t5 - t6
            t3 t6 - t7
            )
            (:predicates 
                (at ?l - t7)
            )
            (:task abs :parameters(?a))
            (:action test1
            :parameters(?l1 ?l2 - t3)
            :precondition (at ?l2)
            :effect (not(at ?l1))
            )
            (:action test2
            :parameters(?l1 ?l2 - t2)
            :precondition (at ?l2)
            :effect (not(at ?l1))
            )
            (:method m1
                :parameters(?l2 - t1 ?l1 - t2 ?l3 - t6)
                :task (abs ?l)
                :precondition ()
                :tasks (and
                    (test1 ?l1 ?l1)
                    (test2 ?l3 ?l1)
                )
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    let semantic_parser = SemanticAnalyzer::new(&ast);
    match semantic_parser.verify_ast() {
        Ok(_) => {
            panic!("error are not caught")
        }
        Err(error) => {
            match error {
                SemanticError::InconsistentTaskArgType(t_error) => {
                    assert_eq!(t_error.expected.unwrap(), "t2");
                    assert_eq!(t_error.found.unwrap(), "t6");
                    assert_eq!(t_error.var_name, "l3");
                    // TODO: test locality
                }
                any => {
                    panic!("{:?}", any)
                }
            }
        }
    }
}