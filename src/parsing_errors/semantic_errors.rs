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
    UndefinedParameter(&'a str),
    // Inconsistency Error
    InconsistentPredicateArity(&'a str),
    InconsistentTaskArity(&'a str),
    InconsistentPredicateArgType(TypeError<'a>),
    InconsistentTaskArgType(TypeError<'a>),
    // Ordering Errors
    CyclicTypeDeclaration(&'a str),
    CyclicOrderingDeclaration(&'a str),
    // Action Errors
    UnsatisfiableActionPrecondition(&'a str),
    // TODO: test
    UnsatisfiableMethodPrecondition(&'a str),
    // TODO: implement
    ImmutablePredicate,
    // Compound Task errors
    // TODO: implement
    NoPrimitiveRefinement(&'a str)
}


#[derive(Debug)]
pub struct TypeError<'a> {
    pub expected: Option<&'a str>,
    pub found: Option<&'a str>,
    pub var_name: &'a str
} 