#[derive(Debug)]
pub enum Token<'a> {
    Keyword(KeywordName),
    Identifier(&'a str),
    Operator(OperationType),
    Punctuator(PunctuationType),
    Requirement(RequirementType)
}

#[derive(Debug)]
pub enum PunctuationType {
    Dash,
    LParentheses,
    RParentheses,
}

#[derive(Debug)]
pub enum OperationType {
    // Logic
    Or,
    Not,
    And,
    Xor,
    ForAll,
    Exists,
    Implication,
    // Ordering
    Equal,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RequirementType {
    MethodPreconditions,
    Hierarchy,
    TypedObjects,
    NegativePreconditions,
    UniversalPreconditions
}

#[derive(Debug)]
pub enum KeywordName {
    Define,
    Domain,
    Problem,
    Requirements,
    Objects,
    Types,
    Task,
    Constants,
    Predicates,
    Init,
    HTN,
    Action,
    Parameters,
    Method,
    Precondition,
    Effect,
    Subtasks, // either "tasks" or "subtasks"
    OrderedSubtasks, // either "ordered-tasks" or "ordered-subtasks"
    Ordering,
    Constraints,
}