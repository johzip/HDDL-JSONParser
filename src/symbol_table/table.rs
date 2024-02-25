use super::*;
use std::collections::{HashSet, HashMap};

pub struct SymbolTable<'a> {
    pub objects: HashSet<&'a str>,
    // TODO: convert to type hierarchy
    pub types: Option<HashSet<&'a str>>,
    pub object_types: Option<HashMap<&'a str, &'a str>>,
    pub requirements: HashSet<RequirementType>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        SymbolTable {
            objects: HashSet::new(),
            types: None,
            object_types: None,
            requirements: HashSet::new()
        }
    }
    pub fn add_object(&mut self, object: &'a str) -> Result<(), SemanticError> {
        if !self.objects.insert(object) {
            return Err(SemanticError {
                error_type: SemanticErrorType::DuplicateObjectDefinition,
            });
        } else {
            Ok(())
        }
    }
    pub fn add_typed_object(
        &mut self,
        object: &'a str,
        object_type: &'a str,
    ) -> Result<(), SemanticError> {
        if !self.objects.insert(object) {
            return Err(SemanticError {
                error_type: SemanticErrorType::DuplicateObjectDefinition,
            });
        } else {
            match &mut self.types {
                Some(types) => {
                    types.insert(object_type);
                    self.object_types.as_mut().unwrap().insert(object, object_type);
                    Ok(())
                }
                None => {
                    self.types = Some(HashSet::from([object_type]));
                    self.object_types = Some(HashMap::from([(object, object_type)]));
                    Ok(())
                }
            }
        }
    }

    pub fn add_requirement(& mut self, req: RequirementType) -> Result<(), SemanticError>{
        if !self.requirements.insert(req) {
            return Err(SemanticError { error_type: SemanticErrorType::DuplicateRequirementDefinition });
        } else {
            Ok(())
        }
    }
}
