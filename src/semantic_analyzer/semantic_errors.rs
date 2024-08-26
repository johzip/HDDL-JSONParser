pub enum SemanticErrorType<'a>{
    // Duplicate Errors
    DuplicateObjectDeclaration(DuplicateError<'a>),
    DuplicateRequirementDeclaration(DuplicateError<'a>),
    DuplicatePredicateDeclaration(DuplicateError<'a>),
    DuplicateActionDeclaration(DuplicateError<'a>),
    DuplicateCompoundTaskDeclaration(DuplicateError<'a>),
    DuplicateMethodDeclaration(DuplicateError<'a>),
    // Parameter Errors
    TypeError,
    UndefinedEntity,
    UnusedElement,
    // Ordering Errors
    CyclicTypeDeclaration,
    CyclicOrderingDeclaration,
}

pub struct DuplicateError<'a>{
    // line number of the initial definition
    pub initial_definition: u32,
    // line number of the duplicate definition
    pub duplicate_definition: u32,
    // duplicate item name
    pub item_name: &'a str
}