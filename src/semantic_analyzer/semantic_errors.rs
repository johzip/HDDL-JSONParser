pub enum SemanticErrorType {
    DuplicateObjectDefinition,
    DuplicateRequirementDefinition,
    DuplicatePredicateDefinition
}

pub struct SemanticError{
    pub error_type: SemanticErrorType,
}