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
                // type checking
                assert_eq!(symbols.object_types.as_ref().unwrap().get(&"a").unwrap(), &"d");
                assert_eq!(symbols.object_types.as_ref().unwrap().get(&"b").unwrap(), &"d");
                assert_eq!(symbols.object_types.as_ref().unwrap().get(&"c").unwrap(), &"d");
                assert_eq!(symbols.object_types.as_ref().unwrap().get(&"s").unwrap(), &"f");
                assert_eq!(symbols.object_types.as_ref().unwrap().contains_key(&"t"), false);
            },
            Err(_) => panic!("parsing errors")
        }
    }

    #[test]
    pub fn untyped_objects_list_test() {
        let program = String::from(
            "(:define (problem p1) (:domain bal) (:objects a b c))"
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(symbols) => {
                assert_eq!(symbols.objects.contains(&"a"), true);
                assert_eq!(symbols.objects.contains(&"b"), true);
                assert_eq!(symbols.objects.contains(&"c"), true);
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
                    let items: Vec<(&str, Option<&str>)> = predicate.variables.arguments
                        .iter()
                        .map(|x| {
                            (x.name, x.var_type)
                        }).collect();
                    if predicate.name == "pred_1" {
                        assert_eq!(
                            items,
                            vec![
                                ("a_1", Some("t_1")), 
                                ("a_2", Some("t_1")),
                                ("a_3", Some("t_2"))
                            ]
                        );
                    } else if predicate.name == "pred_2" {
                        assert_eq!(predicate.variables.arguments.len(), 0);
                    } else if predicate.name == "pred_3" {
                        let items: Vec<(&str, Option<&str>)> = predicate.variables.arguments
                            .iter()
                            .map(|x| {
                                (x.name, x.var_type)
                            }).collect();
                        assert_eq!(
                            items,
                            vec![("a_1", None), ("a_2", None)]
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
    pub fn method_parsing_test() {
        let program = String::from(
            "(:define (:domain bal)
                (:method m_1
                    :parameters (?p1 - p ?l1 ?l2 ?l3 - loc) 
                    :task (deliver_abs ?p1 ?l1 ?l2)
                    :subtasks (and
                        (pickup ?p1 ?l1)
                        (deliver_abs ?p1 ?l2 ?l3)
                    )
                )
             ) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.methods.len(),1);
                let method = &ast.methods[0];
                assert_eq!(method.name, "m_1");
                assert_eq!(method.task_name, "deliver_abs");
                assert_eq!(method.task_terms.arguments.len(), 3);
                assert_eq!(method.task_terms.arguments[0].name, "p1");
                assert_eq!(method.task_terms.arguments[1].name, "l1");
                assert_eq!(method.task_terms.arguments[2].name, "l2");
                assert_eq!(method.tn.subtasks[0].task_symbol, "pickup");
                assert_eq!(method.tn.subtasks[0].terms[0], "p1");
                assert_eq!(method.tn.subtasks[0].terms[1], "l1");
                assert_eq!(method.tn.subtasks[1].task_symbol, "deliver_abs");
                assert_eq!(method.tn.subtasks[1].terms[0], "p1");
                assert_eq!(method.tn.subtasks[1].terms[1], "l2");
                assert_eq!(method.tn.subtasks[1].terms[2], "l3");
            },
            _ => panic!("AST not created")
        }
    }

    // TODO: test constraints
    #[test]
    pub fn init_tn_parsing_test() {
        let program = String::from(
            "(:define (problem p1) (:domain bal)
             (:htn
                :parameters ()
                :subtasks (and
                    (task0 (deliver package_0 city_loc_0))
                    (task1 (retrieve package_1 city_loc_2 truck3))
                )
                :ordering (and
                    (< task0 task1)
                )
            ) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                match ast.init_tn {
                    Some(tn) => {
                        assert_eq!(tn.parameters.is_none(), true);
                        match tn.tn.orderings {
                            TaskOrdering::Partial(o) => {
                                assert_eq!(o.contains(&("task0", "task1")), true);
                                assert_eq!(o.len(), 1);
                            },
                            _ => {
                                panic!("ordering is not total")
                            }
                        }
                        assert_eq!(tn.tn.subtasks.len(), 2);
                        assert_eq!(tn.tn.subtasks[0].id, Some("task0"));
                        assert_eq!(tn.tn.subtasks[0].task_symbol, "deliver");
                        assert_eq!(tn.tn.subtasks[0].terms.len(), 2);
                        assert_eq!(tn.tn.subtasks[0].terms[0], "package_0");
                        assert_eq!(tn.tn.subtasks[0].terms[1], "city_loc_0");
                        assert_eq!(tn.tn.subtasks[1].id, Some("task1"));
                        assert_eq!(tn.tn.subtasks[1].task_symbol, "retrieve");
                        assert_eq!(tn.tn.subtasks[1].terms.len(), 3);
                        assert_eq!(tn.tn.subtasks[1].terms[0], "package_1");
                        assert_eq!(tn.tn.subtasks[1].terms[1], "city_loc_2");
                        assert_eq!(tn.tn.subtasks[1].terms[2], "truck3");
                    },
                    None => {
                        panic!("init tn not parsed")
                    }
                }
            },
            _ => {
                panic!("failed to create AST")
            }
        }
    }

    #[test]
    pub fn init_total_order_tn_parsing_test() {
        let program = String::from(
            "(:define (problem p1) (:domain bal)
             (:htn
                :parameters ()
                :ordered-tasks (and
                    (deliver package_0 city_loc_0)
                    (retrieve package_1 city_loc_2 truck3)
                )
            ) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                match ast.init_tn {
                    Some(tn) => {
                        assert_eq!(tn.parameters.is_none(), true);
                        match tn.tn.orderings {
                            TaskOrdering::Total => { },
                            _ => {
                                panic!("ordering is not partial")
                            }
                        }
                        assert_eq!(tn.tn.subtasks.len(), 2);
                        assert_eq!(tn.tn.subtasks[0].id, None);
                        assert_eq!(tn.tn.subtasks[0].task_symbol, "deliver");
                        assert_eq!(tn.tn.subtasks[0].terms.len(), 2);
                        assert_eq!(tn.tn.subtasks[0].terms[0], "package_0");
                        assert_eq!(tn.tn.subtasks[0].terms[1], "city_loc_0");
                        assert_eq!(tn.tn.subtasks[1].id, None);
                        assert_eq!(tn.tn.subtasks[1].task_symbol, "retrieve");
                        assert_eq!(tn.tn.subtasks[1].terms.len(), 3);
                        assert_eq!(tn.tn.subtasks[1].terms[0], "package_1");
                        assert_eq!(tn.tn.subtasks[1].terms[1], "city_loc_2");
                        assert_eq!(tn.tn.subtasks[1].terms[2], "truck3");
                    },
                    None => panic!("tn not found")
                }
            },
            _ => panic!("false parsing")
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
            Ok(ast) => {
                assert_eq!(ast.compound_tasks.len(), 1);
                let c_1 = &ast.compound_tasks[0];
                assert_eq!(c_1.name, "c_1");
                let c1_term_names: Vec<&str> = c_1.parameters.arguments.iter().map(|x| {
                    x.name
                }).collect();
                let c1_term_types: Vec<&str> = c_1.parameters.arguments.iter().map(|x| {
                    x.var_type.unwrap()
                }).collect();
                assert_eq!(
                    c1_term_names,
                    vec!["p_1", "p_2", "p_3"]
                );
                assert_eq!(
                    c1_term_types,
                    vec!["t1", "t1", "t2"]
                );
            },
            Err(_) => panic!("parsing errors")
        }
    }

    // TODO: add preconditions and effects test
    #[test]
    pub fn action_parsing_test() {
        let program = String::from(
            "(:define (:domain bal)
                (:action a_1
                 :parameters (p_1 p_2 - t1 p_3 - t2)
                 :precondition ()
                 :effect ()
                )
             ) "
        ).into_bytes();
        let lexer = LexicalAnalyzer::new(program);
        match Parser::new(&lexer).parse() {
            Ok(ast) => {
                assert_eq!(ast.actions.len(), 1);
                let action = &ast.actions[0];
                assert_eq!(action.name, "a_1");
                let a1_vars: Vec<&str> = action.parameters.arguments.iter().map(|x| {
                    x.name
                }).collect();
                let a1_var_types: Vec<&str> = action.parameters.arguments.iter().map(|x| {
                    x.var_type.unwrap()
                }).collect();
                assert_eq!(
                    a1_vars,
                    vec!["p_1", "p_2", "p_3"]
                );
                assert_eq!(
                    a1_var_types,
                    vec!["t1", "t1", "t2"]
                );
                match action.preconditions {
                    Formula::Empty => {},
                    _ => panic!("wrong formula")
                }
                match action.effects {
                    Formula::Empty => {},
                    _ => panic!("wrong formula")
                }
            },
            Err(_) => panic!("parsing errors")
        }
    }
}