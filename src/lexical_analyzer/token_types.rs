#[derive(Debug)]
pub enum Token<'a> {
    Keyword(&'a str),
    Identifier(&'a str),
    Operator(OperationType),
    Punctuator(PunctuationType)
}

#[derive(Debug)]
pub enum PunctuationType {
    Dash,
    Colon,
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