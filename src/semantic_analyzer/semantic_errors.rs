use crate::RequirementType;

pub enum SemanticError<'a>{
    // Duplicate Errors
    DuplicateObjectDeclaration(&'a str),
    DuplicateRequirementDeclaration(&'a RequirementType),
    DuplicatePredicateDeclaration(&'a str),
    DuplicateActionDeclaration(&'a str),
    DuplicateCompoundTaskDeclaration(&'a str),
    DuplicateMethodDeclaration(&'a str),
    // Undefined Entities
    // TODO: test implementation
    UndefinedPredicate(&'a str),
    // TODO: test implementation
    UndefinedType(&'a str),
    // TODO: test implementation
    UndefinedTask(&'a str),
    // Parameter Error
    UnusedElement,
    IncompatibleType,
    // TODO: test implementation
    InconsistentPredicateArity(&'a str),
    // Ordering Errors
    CyclicTypeDeclaration,
    CyclicOrderingDeclaration,
}