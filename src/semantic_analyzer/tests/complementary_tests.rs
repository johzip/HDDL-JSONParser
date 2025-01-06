use super::*;

#[test]
pub fn unsat_action_prec_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition (and
                    (at ?l1)
                    (not (at ?l1))
                )
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t4 (p_1 ?p1))
                )
            )
            (:method m_2
                :parameters (?p1) 
                :task (abs_2 ?p1)
                :ordered-subtasks ()
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let semantic_analyzer = DomainSemanticAnalyzer::new(&d);
            match semantic_analyzer.verify_domain() {
                Err(SemanticErrorType::ComplementaryActionPrecondition(t)) => {
                    assert_eq!(t.line, 9)
                }
                _ => panic!(),
            }
        }
        _ => panic!()
    }
}

#[test]
pub fn possibly_unsat_action_prec_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1 ?l2)
            :precondition (and
                    (at ?l1)
                    (not (at ?l2))
                )
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :ordered-subtasks (and
                    (t4 (p_1 ?p1))
                )
            )
            (:method m_2
                :parameters (?p1) 
                :task (abs_2 ?p1)
                :ordered-subtasks ()
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let semantic_analyzer = DomainSemanticAnalyzer::new(&d);
            match semantic_analyzer.verify_domain() {
                Ok(_) => {}
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

#[test]
pub fn unsat_method_prec_test () {
    let program = String::from(
        "(define (domain bal)
            (:predicates 
                (at ?l)
            )
            (:action p_1
            :parameters(?l1)
            :precondition (at ?l1)
            )
            (:action p_2
            :parameters(?l1)
            :precondition ()
            )
            (:task abs_1 :parameters(?a))
            (:task abs_2 :parameters(?a))

            (:method m_1
                :parameters (?p1) 
                :task (abs_1 ?p1)
                :precondition (and
                    (at ?p1)
                    (not (at ?p1))
                )
                :ordered-subtasks (and
                    (t4 (p_1 ?p1))
                )
            )
            (:method m_2
                :parameters (?p1) 
                :task (abs_2 ?p1)
                :ordered-subtasks ()
            )
        ) ",
    )
    .into_bytes();
    let lexer = LexicalAnalyzer::new(&program);
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    match ast {
        AbstractSyntaxTree::Domain(d) => {
            let semantic_analyzer = DomainSemanticAnalyzer::new(&d);
            match semantic_analyzer.verify_domain() {
                Err(SemanticErrorType::ComplementaryMethodPrecondition(t)) => {
                    assert_eq!(t.line, 16)
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}