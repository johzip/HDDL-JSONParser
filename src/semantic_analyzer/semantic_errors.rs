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
    // TODO: implement
    UndefinedTask(&'a str),
    // Parameter Error
    // TODO: implement
    UnusedElement,
    // TODO: implement
    IncompatibleType,
    InconsistentPredicateArity(&'a str),
    // TODO: implement
    InconsistentTaskArity(&'a str),
    // Ordering Errors
    // TODO: implement
    CyclicTypeDeclaration,
    // TODO: implement
    CyclicOrderingDeclaration,
}