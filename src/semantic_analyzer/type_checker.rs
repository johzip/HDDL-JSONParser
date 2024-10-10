use std::collections::{HashMap, HashSet};

use petgraph::algo::{has_path_connecting, toposort};
use petgraph::{prelude::GraphMap, Directed};

use super::*;

pub struct TypeChecker<'a> {
    type_hierarchy: GraphMap<&'a str, (), Directed>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(types: &Option<Vec<Symbol<'a>>>) -> TypeChecker<'a> {
        match &types {
            None => TypeChecker {
                type_hierarchy: GraphMap::new(),
            },
            Some(type_deps) => {
                let mut type_graph: GraphMap<&str, (), Directed> =
                    GraphMap::<_, (), Directed>::new();
                for delcared_type in type_deps {
                    if !type_graph.contains_node(delcared_type.name) {
                        type_graph.add_node(delcared_type.name);
                    }
                    match &delcared_type.symbol_type {
                        None => {}
                        Some(parent) => {
                            if !type_graph.contains_node(parent) {
                                type_graph.add_node(parent);
                            }
                            type_graph.add_edge(delcared_type.name, parent, ());
                        }
                    }
                }
                TypeChecker {
                    type_hierarchy: type_graph,
                }
            }
        }
    }

    pub fn check_acyclicity(&self) -> Option<SemanticErrorType> {
        match toposort(&self.type_hierarchy, None) {
            Ok(_) => None,
            Err(cycle_item) => {
                let node = cycle_item.node_id();
                return Some(SemanticErrorType::CyclicTypeDeclaration(node.to_string()));
            }
        }
    }

    pub fn check_type_declarations(
        &self,
        parameters: &Vec<Symbol<'a>>,
    ) -> Option<SemanticErrorType> {
        for parameter in parameters.iter() {
            if let Some(t) = parameter.symbol_type {
                if !self.type_hierarchy.contains_node(t) {
                    return Some(SemanticErrorType::UndefinedType(t.to_string()));
                }
            }
        }
        None
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
        if let Some(undeclared_type) = self.check_type_declarations(parameters) {
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
                                    return Err(SemanticErrorType::UndefinedParameter(var.name.to_string()));
                                } else {
                                    found_list.push((var.name, &declared_constants.get(&var.name).unwrap().symbol_type))
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
                        return Err(SemanticErrorType::InconsistentPredicateArity(
                            used_predicate.name.to_string(),
                        ));
                    }
                    for ((var_name, f), e) in found_list.into_iter().zip(expected_list.into_iter())
                    {
                        if !self.is_var_type_consistent(*f, *e) {
                            return Err(SemanticErrorType::InconsistentPredicateArgType(TypeError {
                                expected: e.map(|inner| inner.to_string()),
                                found: f.map(|inner| inner.to_string()),
                                var_name: var_name.to_string(),
                            }));
                        }
                    }
                }
                None => {
                    return Err(SemanticErrorType::UndefinedPredicate(used_predicate.name.to_string()));
                }
            }
        }
        Ok(())
    }

    fn is_var_type_consistent(&self, found: Option<&'a str>, expected: Option<&'a str>) -> bool {
        match (found, expected) {
            (Some(found_typing), Some(defined_typing)) => {
                // type matches exactly
                if found_typing == defined_typing {
                    return true;
                }
                // search whether there is a path from current type to a super type
                if !has_path_connecting(&self.type_hierarchy, found_typing, defined_typing, None) {
                    return false;
                } else {
                    return true;
                }
            }
            (None, None) => {
                return true;
            }
            (None, Some(_)) => return false,
            (Some(_), None) => {
                return false;
            }
        }
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
            HashMap::from_iter(parameters.iter().map(|par| {
                (par.name, par.symbol_type)
            }));
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
                let expected: Vec<Option<&str>> =
                    definition.parameters.iter().map(|x| x.symbol_type).collect();
                if found.len() != expected.len() {
                    return Err(SemanticErrorType::InconsistentTaskArity(task_name.to_string()));
                }
                for ((name, f), e) in found.iter().zip(expected.iter()) {
                    if !self.is_var_type_consistent(**f, *e) {
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
                    let expected: Vec<Option<&str>> =
                        definition.parameters.iter().map(|x| x.symbol_type).collect();
                    if found.len() != expected.len() {
                        return Err(SemanticErrorType::InconsistentTaskArity(task_name.to_string()));
                    }
                    for ((name, f), e) in found.iter().zip(expected.iter()) {
                        if !self.is_var_type_consistent(**f, *e) {
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
