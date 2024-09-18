use std::collections::HashSet;

use crate::lexical_analyzer::RequirementType;

use super::*;

pub fn check_duplicate_objects<'a>(objects: &'a Vec<Symbol<'a>>) -> Option<SemanticErrorType<'a>> {
    let mut names = HashSet::new();
    for obj in objects {
        if !names.insert(obj.name) {
            return Some(SemanticErrorType::DuplicateObjectDeclaration(&obj.name));
        }
    }
    None
}

pub fn check_duplicate_requirements<'a>(requirements: &'a Vec<RequirementType>) -> Option<SemanticErrorType<'a>> {
    let mut names = HashSet::new();
    for req in requirements {
        if !names.insert(req) {
            return Some(SemanticErrorType::DuplicateRequirementDeclaration(req));
        }
    }
    None
}