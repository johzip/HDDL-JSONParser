use std::collections::HashSet;

use petgraph::{algo::{toposort, Cycle}, prelude::GraphMap, Directed};

use super::*;

pub fn check_ordering_acyclic<'a>(tn: &'a HTN<'a>) -> Result<(), SemanticError<'a>> {
    match &tn.orderings {
        TaskOrdering::Total => {
            return Ok(());
        }
        TaskOrdering::Partial(orderings) => {
            let ordering_graph = GraphMap::<_, (), Directed>::from_edges(orderings);
            match toposort(&ordering_graph, None) {
                Ok(_) => {
                    return Ok(());
                }
                Err(cycle_item) => {
                    return Err(SemanticError::CyclicOrderingDeclaration(cycle_item.node_id()));
                }
            }
        }
    }
}

pub fn check_types_acyclic<'a>(types: Vec<Variable<'a>>) -> Result<(), SemanticError<'a>> {
    todo!()
}