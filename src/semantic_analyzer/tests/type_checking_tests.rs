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
    let lexer = LexicalAnalyzer::new(program);
    let parser = Parser::new(&lexer);
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
    let lexer = LexicalAnalyzer::new(program);
    let parser = Parser::new(&lexer);
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
                :parameters(?l2 - t1 ?l1)
                :task (abs ?l)
                :precondition (at ?l2)
                :tasks (test1 ?l2 ?l2)
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(program);
    let parser = Parser::new(&lexer);
    let ast = parser.parse().unwrap();
    let semantic_parser = SemanticAnalyzer::new(&ast);
    match semantic_parser.verify_ast() {
        Ok(_) => {        }
        Err(error) => {
            panic!("{:?}", error)
        }
    }
}