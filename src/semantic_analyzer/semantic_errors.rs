use crate::RequirementType;

pub enum SemanticError<'a>{
    // Duplicate Errors
    DuplicateObjectDeclaration(&'a str),
    DuplicateRequirementDeclaration(&'a RequirementType),
    DuplicatePredicateDeclaration(&'a str),
    DuplicateActionDeclaration(&'a str),
    DuplicateCompoundTaskDeclaration(&'a str),
    DuplicateMethodDeclaration(&'a str),
    // Parameter Errors
    TypeError,
    UndefinedEntity,
    UnusedElement,
    // Ordering Errors
    CyclicTypeDeclaration,
    CyclicOrderingDeclaration,
}