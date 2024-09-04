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
    InconsistentPredicateArity(&'a str),
    InconsistentTaskArity(&'a str),
    // TODO: implement
    InconsistentPredicateArgType,
    // TODO: implement
    InconsistentTaskArgType,
    // Ordering Errors
    CyclicTypeDeclaration(&'a str),
    CyclicOrderingDeclaration(&'a str),
    // Redundant Elements
    // TODO: implement
    UnusedType(&'a str),
    // TODO: implement
    UnusedPredicate(&'a str),
    // TODO: implement
    UnusedParameter(&'a str),
    // Complementary Predicates
    // TODO: implement
    ComplementaryEffects(&'a str),
    // TODO: implement
    ComplementaryPreconditions(&'a str),
}