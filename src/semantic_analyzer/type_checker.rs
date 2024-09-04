use petgraph::{prelude::GraphMap, Directed};
use petgraph::algo::toposort;

use super::*;

pub struct TypeChecker<'a> {
    type_hierarchy: GraphMap<&'a str, (), Directed>,
}

impl <'a> TypeChecker <'a> {
    pub fn new(types: &Option<Vec<Variable<'a>>>) -> TypeChecker<'a> {
        match &types {
            None => {
                TypeChecker {
                    type_hierarchy: GraphMap::new()
                }
            },
            Some(type_deps) => {
                let mut type_graph: GraphMap<&str, (), Directed> = GraphMap::<_, (), Directed>::new();
                for delcared_type in type_deps {
                    if !type_graph.contains_node(delcared_type.name) {
                        type_graph.add_node(delcared_type.name);
                    }
                    match &delcared_type.var_type {
                        None => {},
                        Some(parent) => {
                            if !type_graph.contains_node(parent) {
                                type_graph.add_node(parent);
                            }
                            type_graph.add_edge(delcared_type.name, parent, ());
                        }
                    }
                }
                TypeChecker {
                    type_hierarchy: type_graph
                }
            }
        }
    }

    pub fn check_acyclicity(&self) -> Option<SemanticError<'a>> {
        match toposort(&self.type_hierarchy, None) {
            Ok(_) => {None},
            Err(cycle_item) => {
                let node = cycle_item.node_id();
                return Some(SemanticError::CyclicTypeDeclaration(node));
            }
        }
    }

    pub fn check_type_declarations(&self, parameters: &Vec<Variable<'a>>) -> Option<SemanticError<'a>> {
        for parameter in parameters.iter() {
            if let Some(t) = parameter.var_type {
                if !self.type_hierarchy.contains_node(t) {
                    return Some(SemanticError::UndefinedType(t));
                }
            }
        }
        None
    }

    // TODO:
    pub fn check_formula(&self, formula: Formula<'a>, parameters: &Vec<Variable<'a>>) -> Result<(), SemanticError<'a>> {
        todo!()
    }
}
