use std::collections::HashSet;

use crate::RequirementType;

use super::*;

pub fn check_duplicate_objects<'a>(objects: &'a Vec<Variable<'a>>) -> Option<SemanticError<'a>> {
    let mut names = HashSet::new();
    for obj in objects {
        if !names.insert(obj.name) {
            return Some(SemanticError::DuplicateObjectDeclaration(&obj.name));
        }
    }
    None
}

pub fn check_duplicate_requirements<'a>(requirements: &'a Vec<RequirementType>) -> Option<SemanticError<'a>> {
    let mut names = HashSet::new();
    for req in requirements {
        if !names.insert(req) {
            return Some(SemanticError::DuplicateRequirementDeclaration(req));
        }
    }
    None
}