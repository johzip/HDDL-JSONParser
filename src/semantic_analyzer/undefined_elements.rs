use std::{collections::HashSet, ops::Sub};
use petgraph::{prelude::GraphMap, Directed};
use super::*;

pub fn check_predicate_declarations<'a>(
    formula: &Formula<'a>,
    declared_predicates: &Vec<Predicate<'a>>,
) -> Result<(), SemanticError<'a>> {
    match &*formula {
        Formula::Empty => {}
        Formula::Atom(predicate) => {
            for declared_predicate in declared_predicates {
                // Assert same name
                if predicate.name == declared_predicate.name {
                    // Assert same arity
                    if predicate.variables.len() == declared_predicate.variables.len() {
                        return Ok(());
                    } else {
                        return Err(SemanticError::InconsistentPredicateArity(predicate.name));
                    }
                }
            }
            return Err(SemanticError::UndefinedPredicate(&predicate.name));
        }
        Formula::Not(new_formula) => {
            return check_predicate_declarations(&*new_formula, declared_predicates);
        }
        Formula::And(new_formula) |
        Formula::Or(new_formula) |
        Formula::Xor(new_formula) => {
            for f in new_formula {
                check_predicate_declarations(&*f, declared_predicates)?;
            }
        }
        Formula::ForAll(_, new_formula) => {
            return check_predicate_declarations(&*new_formula, declared_predicates);
        }
        Formula::Equals(_, _) => {}
        // TODO: add support for imply, and exists
        _ => {
            panic!()
        }
    }
    return Ok(());
}

pub fn check_subtask_declarations<'a>(
    subtasks: &Vec<Subtask<'a>>,
    declared_compound_tasks: &Vec<Task<'a>>,
    declared_actions: &Vec<Action<'a>>,
) -> Result<(), SemanticError<'a>> {
    for task in subtasks.iter() {
        let mut is_compound = false;
        for declared_compound_task in declared_compound_tasks.iter() {
            if task.task_symbol == declared_compound_task.name {
                if task.terms.len() != declared_compound_task.parameters.len() {
                    return Err(SemanticError::InconsistentTaskArity(&task.task_symbol));
                } else {
                    is_compound = true;
                    break;
                }
            }
        }
        let mut is_primitive = false;
        if !is_compound {
            for declared_action in declared_actions.iter() {
                if task.task_symbol == declared_action.name {
                    if task.terms.len() != declared_action.parameters.len() {
                        return Err(SemanticError::InconsistentTaskArity(&task.task_symbol));
                    } else {
                        is_primitive = true;
                        break;
                    }
                }
            }
        }
        if !is_primitive && !is_compound{
            return Err(SemanticError::UndefinedSubtask(task.task_symbol));
        }
    }
    Ok(())
}

pub fn check_type_declarations<'a>(
    parameters: &Vec<Variable<'a>>,
    declared_types: &Option<GraphMap<&str, (), Directed>>,
) -> Result<(), SemanticError<'a>> {
    match &declared_types {
        Some(graph) => {
            for parameter in parameters.iter() {
                match &parameter.var_type {
                    Some(t) => {
                        if !graph.contains_node(t) {
                            return Err(SemanticError::UndefinedType(t));
                        }
                    }
                    _ => {}
                }
            }
        }
        None => {
            for parameter in parameters.iter() {
                match &parameter.var_type {
                    Some(t) => {
                        return Err(SemanticError::UndefinedType(&t));
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
