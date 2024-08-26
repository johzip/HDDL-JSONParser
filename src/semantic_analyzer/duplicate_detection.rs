use std::collections::HashSet;

use crate::RequirementType;

use super::*;

pub fn check_duplicate_objects<'a>(objects: &'a Vec<Variable<'a>>) -> Result<(), SemanticError<'a>> {
    let mut names = HashSet::new();
    for obj in objects {
        if !names.insert(obj.name) {
            return Err(SemanticError::DuplicateObjectDeclaration(&obj.name));
        }
    }
    Ok(())
}

pub fn check_duplicate_requirements<'a>(requirements: &'a Vec<RequirementType>) -> Result<(), SemanticError<'a>> {
    let mut names = HashSet::new();
    for req in requirements {
        if !names.insert(req) {
            return Err(SemanticError::DuplicateRequirementDeclaration(req));
        }
    }
    Ok(())
}

pub fn check_duplicate_predicates<'a>(predicates: &'a Vec<Predicate<'a>>) -> Result<(), SemanticError<'a>> {
    let mut names = HashSet::new();
    for pred in predicates {
        if !names.insert(pred.name) {
            return Err(SemanticError::DuplicatePredicateDeclaration(&pred.name));
        }
    }
    Ok(())
}

pub fn check_duplicate_actions<'a>(actions: &'a Vec<Action<'a>>) -> Result<(), SemanticError<'a>> {
    let mut names = HashSet::new();
    for act in actions {
        if !names.insert(act.name) {
            return Err(SemanticError::DuplicateActionDeclaration(&act.name));
        }
    }
    Ok(())
}

pub fn check_duplicate_compound_tasks<'a>(tasks: &'a Vec<Task<'a>>) -> Result<(), SemanticError<'a>> {
    let mut names = HashSet::new();
    for task in tasks {
        if !names.insert(task.name) {
            return Err(SemanticError::DuplicateCompoundTaskDeclaration(&task.name));
        }
    }
    Ok(())
}

pub fn check_duplicate_methods<'a>(methods: &'a Vec<Method<'a>>) -> Result<(), SemanticError<'a>> {
    let mut names = HashSet::new();
    for method in methods {
        if !names.insert(method.name) {
            return Err(SemanticError::DuplicateMethodDeclaration(&method.name));
        }
    }
    Ok(())
}