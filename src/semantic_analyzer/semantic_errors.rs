use crate::RequirementType;

#[derive(Debug)]
pub enum SemanticError<'a>{
    // Duplicate Errors
    DuplicateObjectDeclaration(&'a str),
    DuplicateRequirementDeclaration(&'a RequirementType),
    DuplicatePredicateDeclaration(&'a str),
    DuplicateActionDeclaration(&'a str),
    DuplicateCompoundTaskDeclaration(&'a str),
    DuplicateMethodDeclaration(&'a str),
    // Undefined Entities
    UndefinedPredicate(&'a str),
    UndefinedType(&'a str),
    UndefinedSubtask(&'a str),
    UndefinedTask(&'a str),
    // Inconsistency Error
    // TODO: implement
    IncompatibleType,
    InconsistentPredicateArity(&'a str),
    InconsistentTaskArity(&'a str),
    // Ordering Errors
    // TODO: implement
    CyclicTypeDeclaration,
    // TODO: implement
    CyclicOrderingDeclaration,
    // Redundant Elements
    // TODO: implement
    UnusedElement,
}