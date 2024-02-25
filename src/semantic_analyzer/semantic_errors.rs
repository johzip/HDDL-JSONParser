pub enum SemanticErrorType {
    DuplicateObjectDefinition,
    DuplicateRequirementDefinition,
}

pub struct SemanticError{
    pub error_type: SemanticErrorType,
}