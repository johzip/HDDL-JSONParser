use std::collections::{HashMap, HashSet};

use petgraph::algo::{has_path_connecting, toposort};
use petgraph::{prelude::GraphMap, Directed};

use super::*;

pub struct TypeChecker<'a> {
    type_hierarchy: GraphMap<&'a str, (), Directed>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(types: &Option<Vec<Variable<'a>>>) -> TypeChecker<'a> {
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
                    match &delcared_type.var_type {
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

    pub fn check_acyclicity(&self) -> Option<SemanticError<'a>> {
        match toposort(&self.type_hierarchy, None) {
            Ok(_) => None,
            Err(cycle_item) => {
                let node = cycle_item.node_id();
                return Some(SemanticError::CyclicTypeDeclaration(node));
            }
        }
    }

    pub fn check_type_declarations(
        &self,
        parameters: &Vec<Variable<'a>>,
    ) -> Option<SemanticError<'a>> {
        for parameter in parameters.iter() {
            if let Some(t) = parameter.var_type {
                if !self.type_hierarchy.contains_node(t) {
                    return Some(SemanticError::UndefinedType(t));
                }
            }
        }
        None
    }

    pub fn check_formula(
        &self,
        formula: &Vec<&Predicate<'a>>,
        parameters: &Vec<Variable<'a>>,
        declared_predicates: &HashSet<&'a Predicate<'a>>,
    ) -> Result<(), SemanticError<'a>> {
        // Assert all types are declared
        if let Some(undeclared_type) = self.check_type_declarations(parameters) {
            return Err(undeclared_type);
        }
        // Store parameter types
        let par_types: HashMap<&str, Option<&str>> =
            HashMap::from_iter(parameters.iter().map(|par| (par.name, par.var_type)));
        // Assert predicate typing correctness
        for used_predicate in formula {
            match declared_predicates.get(used_predicate) {
                Some(predicate_definition) => {
                    if predicate_definition.variables.len() != used_predicate.variables.len() {
                        return Err(SemanticError::InconsistentPredicateArity(
                            &used_predicate.name,
                        ));
                    }
                    let zipped_args = used_predicate
                        .variables
                        .iter()
                        .zip(predicate_definition.variables.iter());
                    for (used, expected) in zipped_args {
                        match par_types.get(used.name) {
                            Some(used_type) => {
                                match self.check_var_type(&used.name, *used_type, expected.var_type) {
                                    None => {continue;}
                                    Some(mistyping) => {
                                        return Err(mistyping);
                                    }
                                }
                            }
                            None => {
                                return Err(SemanticError::UndefinedParameter(&used.name));
                            }
                        }
                    }
                }
                None => {
                    return Err(SemanticError::UndefinedPredicate(&used_predicate.name));
                }
            }
        }
        Ok(())
    }

    fn check_var_type(&self, var_name: &'a str, found: Option<&'a str>, expected: Option<&'a str>) -> Option<SemanticError<'a>> {
        match (found, expected) {
            (Some(found_typing), Some(defined_typing)) => {
                // type matches exactly
                if found_typing == defined_typing {
                    return None;
                }
                // search whether there is a path from current type to a super type
                if !has_path_connecting(&self.type_hierarchy, found_typing, defined_typing, None) {
                    return Some(SemanticError::InconsistentPredicateArgType(TypeError {
                        expected: expected,
                        found: found,
                        var_name: var_name,
                    }));
                } else {
                    return None;
                }
            }
            (None, None) => {
                return None;
            }
            (None, Some(_)) => {
                return Some(SemanticError::InconsistentPredicateArgType(TypeError {
                    expected: expected,
                    found: None,
                    var_name: var_name,
                }));
            }
            (Some(_), None) => {
                return Some(SemanticError::InconsistentPredicateArgType(TypeError {
                    expected: None,
                    found: found,
                    var_name: var_name,
                }));
            }
        }
    }
}
