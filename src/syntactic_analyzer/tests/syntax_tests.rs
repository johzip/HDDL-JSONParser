use super::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore = "stupid test, rewrite from scratch"]
    pub fn file_type_test() {
        let program = String::from("(:define (domain jajaja) (:predicates ()) ) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        let parser = Parser::new(&lexer);
        match parser.parse() {
            Ok(_) => {},
            _ => panic!("parsing error")
        }
        let program = String::from("(:define (problem jajaja2) (:domain blahblah)) ").into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        let parser = Parser::new(&lexer);
        match parser.parse() {
            Ok(_) => {},
            _ => panic!("parsing error")
        }
    }

    #[test]
    pub fn objects_list_test() {
        let program = String::from(
            "(:define (problem p1) (:domain bal) (:objects a b c - d s - f t))"
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.objects.contains(&"a"), true);
                assert_eq!(symbols.objects.contains(&"b"), true);
                assert_eq!(symbols.objects.contains(&"c"), true);
                assert_eq!(symbols.objects.contains(&"s"), true);
                assert_eq!(symbols.objects.contains(&"t"), true);
            },
            Err(_) => panic!("parsing errors")
        }
    }

    #[test]
    pub fn requirement_parsing_test() {
        let program = String::from(
            "(:define (problem p1) (:domain bal)
             (:requirements :hierarchy :method-preconditions :typing :negative-preconditions)) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.requirements.len(), 4);
                assert_eq!(symbols.requirements.contains(&RequirementType::Hierarchy), true);
                assert_eq!(symbols.requirements.contains(&RequirementType::MethodPreconditions), true);
                assert_eq!(symbols.requirements.contains(&RequirementType::NegativePreconditions), true);
                assert_eq!(symbols.requirements.contains(&RequirementType::TypedObjects), true);
            },
            Err(_) => panic!("parsing errors")
        }
    }

    #[test]
    pub fn predicate_parsing_test() {
        let program = String::from(
            "(:define (:domain bal)
                (:predicates 
                    (pred_1 ?a_1 ?a_2 - t_1 ?a_3 - t_2)
                    (pred_2)
                    (pred_3 a_1 a_2)
                )
             ) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.predicates.len(), 3);
                for predicate in symbols.predicates {
                    if predicate.name == "pred_1" {
                        assert_eq!(
                            predicate.variables.variables,
                            vec!["a_1", "a_2", "a_3"]
                        );
                        assert_eq!(
                            predicate.variables.variable_types.as_ref().unwrap().len(),
                            3
                        );
                        assert_eq!(
                            predicate.variables.variable_types.as_ref().unwrap().get("a_1").unwrap(),
                            &"t_1"
                        );
                        assert_eq!(
                            predicate.variables.variable_types.as_ref().unwrap().get("a_2").unwrap(),
                            &"t_1"
                        );
                        assert_eq!(
                            predicate.variables.variable_types.as_ref().unwrap().get("a_3").unwrap(),
                            &"t_2"
                        );
                    } else if predicate.name == "pred_2" {
                        assert_eq!(predicate.variables.variables.len(), 0);
                    } else if predicate.name == "pred_3" {
                        assert_eq!(
                            predicate.variables.variables,
                            vec!["a_1", "a_2"]
                        );
                        assert_eq!(
                            predicate.variables.variable_types,
                            None
                        );
                    } else {
                        panic!("parsing error")
                    }
                }                
            },
            Err(_) => panic!("parsing errors")
        }
    }

    #[test]
    pub fn compound_task_parsing_test() {
        let program = String::from(
            "(:define (:domain bal)
                (:task c_1
                    :parameters (p_1 p_2 - t1 p_3 - t2)
                )
             ) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.compound_tasks.len(), 1);
                let c_1 = &symbols.compound_tasks[0];
                assert_eq!(c_1.name, "c_1");
                assert_eq!(
                    c_1.parameters.variables,
                    vec!["p_1", "p_2", "p_3"]
                );
                assert_eq!(
                    c_1.parameters.variable_types.as_ref().unwrap().len(),
                    3
                );
                assert_eq!(
                    c_1.parameters.variable_types.as_ref().unwrap().get("p_1").unwrap(),
                    &"t1"
                );
                assert_eq!(
                    c_1.parameters.variable_types.as_ref().unwrap().get("p_2").unwrap(),
                    &"t1"
                );
                assert_eq!(
                    c_1.parameters.variable_types.as_ref().unwrap().get("p_3").unwrap(),
                    &"t2"
                );
            },
            Err(_) => panic!("parsing errors")
        }
    }
}