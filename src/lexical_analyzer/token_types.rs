use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Keyword(KeywordName),
    Identifier(&'a str),
    Operator(OperationType),
    Punctuator(PunctuationType),
    Requirement(RequirementType),
    EOF
}

#[derive(Debug, PartialEq, Eq)]
pub enum PunctuationType {
    Dash,
    LParentheses,
    RParentheses,
}

impl fmt::Display for PunctuationType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PunctuationType::Dash => write!(fmt, "-"),
            PunctuationType::LParentheses => write!(fmt, "("),
            PunctuationType::RParentheses => write!(fmt, ")"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

impl fmt::Display for OperationType {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            OperationType::Or => write!(fmt, "or"),
            OperationType::Not => write!(fmt, "not"),
            OperationType::And => write!(fmt, "and"),
            OperationType::Xor => write!(fmt, "oneof"),
            OperationType::ForAll => write!(fmt, "forall"),
            OperationType::Exists => write!(fmt, "exists"),
            OperationType::Implication => write!(fmt, "when"),
            OperationType::Equal => write!(fmt, "="),
            OperationType::LessThan => write!(fmt, "<"),
            OperationType::GreaterThan => write!(fmt, ">"),
            OperationType::LessThanOrEqual => write!(fmt, "<="),
            OperationType::GreaterThanOrEqual => write!(fmt, ">="),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RequirementType {
    MethodPreconditions,
    Hierarchy,
    TypedObjects,
    NegativePreconditions,
    UniversalPreconditions
}

#[derive(Debug, PartialEq, Eq)]
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