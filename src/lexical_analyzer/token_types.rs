#[derive(Debug)]
pub enum Token<'a> {
    Keyword(KeywordName),
    Identifier(&'a str),
    Operator(OperationType),
    Punctuator(PunctuationType)
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

#[derive(Debug)]
pub enum KeywordName {
    Define,
    Domain,
    Problem,
    Requirements,
    Objects,
    Types,
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

impl <'a> Token<'a> {
    pub fn is_l_parantheses(&self) -> bool {
        match &self {
            Token::Punctuator(PunctuationType::LParentheses) => true,
            _ => false
        }
    }
    pub fn is_r_parantheses(&self) -> bool {
        match &self {
            Token::Punctuator(PunctuationType::RParentheses) => true,
            _ => false
        }
    }
}