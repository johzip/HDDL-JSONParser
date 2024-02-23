pub enum SemanticErrorType {
    DuplicateObjectDefinition
}

pub struct SemanticError{
    pub error_type: SemanticErrorType,
}