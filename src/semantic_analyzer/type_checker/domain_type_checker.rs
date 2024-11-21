use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Clone)]
pub struct DomainTypeChecker<'a> {
    pub(super) generic_type_checker: TypeChecker<'a>,
}

impl<'a> DomainTypeChecker<'a> {
    pub fn new(types: &Option<Vec<Symbol<'a>>>) -> DomainTypeChecker<'a> {
        DomainTypeChecker {
            generic_type_checker: TypeChecker::new(types),
        }
    }

    pub fn get_type_hierarchy(&'a self) -> GraphMap<&'a str, (), Directed> {
        self.generic_type_checker.type_hierarchy.clone()
    }

    pub fn check_type_declarations(
        &self,
        parameters: &Vec<Symbol<'a>>,
    ) -> Option<SemanticErrorType> {
        self.generic_type_checker
            .check_type_declarations(parameters)
    }

    pub fn verify_type_hierarchy(&self) -> Result<(), SemanticErrorType> {
        self.generic_type_checker.verify_type_hierarchy()
    }

    // TODO: Add support for "universal qunatification" parameters
    pub fn check_formula(
        &self,
        formula: &Vec<&Predicate<'a>>,
        parameters: &Vec<Symbol<'a>>,
        declared_constants: &HashSet<&Symbol<'a>>,
        declared_predicates: &HashSet<&'a Predicate<'a>>,
    ) -> Result<(), SemanticErrorType> {
        // Assert all types are declared
        if let Some(undeclared_type) = self
            .generic_type_checker
            .check_type_declarations(parameters)
        {
            return Err(undeclared_type);
        }
        // Store parameter types
        let par_types: HashMap<&str, Option<&str>> =
            HashMap::from_iter(parameters.iter().map(|par| (par.name, par.symbol_type)));
        // Assert predicate typing correctness
        for used_predicate in formula {
            match declared_predicates.get(used_predicate) {
                Some(predicate_definition) => {
                    let mut found_list = vec![];
                    for var in used_predicate.variables.iter() {
                        match par_types.get(var.name) {
                            Some(par_type) => {
                                found_list.push((var.name, par_type));
                            }
                            None => {
                                if !declared_constants.contains(&var.name) {
                                    return Err(SemanticErrorType::UndefinedParameter(
                                        var.name.to_string(),
                                    ));
                                } else {
                                    found_list.push((
                                        var.name,
                                        &declared_constants.get(&var.name).unwrap().symbol_type,
                                    ))
                                }
                            }
                        }
                    }
                    let mut expected_list: Vec<&Option<&str>> = predicate_definition
                        .variables
                        .iter()
                        .map(|x| &x.symbol_type)
                        .collect();
                    // Assert args have the same arity
                    if &found_list.len() != &expected_list.len() {
                        return Err(SemanticErrorType::InconsistentPredicateArity(ArityError {
                            symbol: used_predicate.name.to_string(),
                            expected_arity: expected_list.len() as u32,
                            found_arity: found_list.len() as u32,
                        }));
                    }
                    for ((var_name, f), e) in found_list.into_iter().zip(expected_list.into_iter())
                    {
                        if !self.generic_type_checker.is_var_type_consistent(*f, *e) {
                            return Err(SemanticErrorType::InconsistentPredicateArgType(
                                TypeError {
                                    expected: e.map(|inner| inner.to_string()),
                                    found: f.map(|inner| inner.to_string()),
                                    var_name: var_name.to_string(),
                                },
                            ));
                        }
                    }
                }
                None => {
                    return Err(SemanticErrorType::UndefinedPredicate(
                        UndefinedSymbolError {
                            symbol: used_predicate.name.to_string(),
                            position: used_predicate.name_pos,
                        },
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn is_task_consistent(
        &self,
        task_name: &'a str,
        task_terms: &Vec<&'a str>,
        parameters: &Vec<Symbol<'a>>,
        declared_constants: &HashSet<&Symbol<'a>>,
        declared_tasks: &HashSet<&Task<'a>>,
        declared_actions: &HashSet<&Action<'a>>,
    ) -> Result<(), SemanticErrorType> {
        // Store parameter types
        let par_types: HashMap<&str, Option<&str>> =
            HashMap::from_iter(parameters.iter().map(|par| (par.name, par.symbol_type)));
        let mut found = vec![];
        for term in task_terms {
            match par_types.get(term) {
                Some(typing) => {
                    found.push((term, typing));
                }
                None => {
                    if !declared_constants.contains(term) {
                        return Err(SemanticErrorType::UndefinedParameter(term.to_string()));
                    } else {
                        found.push((term, &declared_constants.get(term).unwrap().symbol_type))
                    }
                }
            }
        }
        match declared_actions.iter().find(|x| x.name == task_name) {
            Some(definition) => {
                let expected: Vec<Option<&str>> = definition
                    .parameters
                    .iter()
                    .map(|x| x.symbol_type)
                    .collect();
                if found.len() != expected.len() {
                    return Err(SemanticErrorType::InconsistentTaskArity(ArityError {
                        symbol: task_name.to_string(),
                        expected_arity: expected.len() as u32,
                        found_arity: found.len() as u32,
                    }));
                }
                for ((name, f), e) in found.iter().zip(expected.iter()) {
                    if !self.generic_type_checker.is_var_type_consistent(**f, *e) {
                        return Err(SemanticErrorType::InconsistentTaskArgType(TypeError {
                            expected: e.map(|inner| inner.to_string()),
                            found: f.map(|inner| inner.to_string()),
                            var_name: name.to_string(),
                        }));
                    }
                }
                return Ok(());
            }
            None => match declared_tasks.iter().find(|x| x.name == task_name) {
                Some(definition) => {
                    let expected: Vec<Option<&str>> = definition
                        .parameters
                        .iter()
                        .map(|x| x.symbol_type)
                        .collect();
                    if found.len() != expected.len() {
                        return Err(SemanticErrorType::InconsistentTaskArity(ArityError {
                            symbol: task_name.to_string(),
                            expected_arity: expected.len() as u32,
                            found_arity: found.len() as u32,
                        }));
                    }
                    for ((name, f), e) in found.iter().zip(expected.iter()) {
                        if !self.generic_type_checker.is_var_type_consistent(**f, *e) {
                            return Err(SemanticErrorType::InconsistentTaskArgType(TypeError {
                                expected: e.map(|inner| inner.to_string()),
                                found: f.map(|inner| inner.to_string()),
                                var_name: name.to_string(),
                            }));
                        }
                    }
                    return Ok(());
                }
                None => {
                    return Err(SemanticErrorType::UndefinedSubtask(task_name.to_string()));
                }
            },
        }
    }
}
