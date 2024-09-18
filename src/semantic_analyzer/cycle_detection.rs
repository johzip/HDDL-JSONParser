use std::collections::HashSet;

use petgraph::{algo::{toposort, Cycle}, prelude::GraphMap, Directed};

use super::*;

pub fn check_ordering_acyclic<'a>(tn: &'a HTN<'a>) -> Result<(), SemanticErrorType<'a>> {
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
                    return Err(SemanticErrorType::CyclicOrderingDeclaration(cycle_item.node_id()));
                }
            }
        }
    }
}