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