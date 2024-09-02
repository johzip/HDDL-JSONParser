use analyzer::verify_semantics;
use super::*;

#[test]
pub fn cyclic_method_ordering_test() {
    let program = String::from(
        "(define (domain bal)
                (:predicates 
                    (hold ?a_1 ?a_2)
                    (pred_2)
                    (at ?a_1)
                )
                (:task deliver_abs_1 :parameters(?p1))
                (:task deliver_abs_2 :parameters(?p1))
                (:task deliver_abs_3 :parameters(?p1))
                (:task deliver_abs_4 :parameters(?p1))

                (:method m_1
                    :parameters (?p1 ?p2 ?p3 ?p4) 
                    :task (deliver_abs_1 ?p1)
                    :subtasks (and
                        (t1 (deliver_abs_1 ?p1))
                        (t2 (deliver_abs_2 ?p2))
                        (t3 (deliver_abs_3 ?p3))
                        (t4 (deliver_abs_4 ?p4))
                    )
                    :ordering (and
                        (< t1 t2)
                        (< t2 t3)
                        (< t3 t4)
                        (< t4 t1)
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
                SemanticError::CyclicOrderingDeclaration(x) => {
                    assert_eq!(x, "t4")
                    // TODO: assert locality in future
                }
                _ => {
                    panic!("caught wrong error")
                }
            }
        }
    }
}