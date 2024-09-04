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
    // TODO test implementation
    UndefinedParameter(&'a str),
    // Inconsistency Error
    InconsistentPredicateArity(&'a str),
    InconsistentTaskArity(&'a str),
    // TODO: implement
    InconsistentPredicateArgType(TypeError<'a>),
    // TODO: implement
    InconsistentTaskArgType(TypeError<'a>),
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


#[derive(Debug)]
pub struct TypeError<'a> {
    pub expected: Option<&'a str>,
    pub found: Option<&'a str>,
    pub var_name: &'a str
} 