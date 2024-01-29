#[derive(Debug)]
pub enum Token<'a> {
    Keyword(&'a str),
    ObjectType(&'a str),
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
    Or,
    Not,
    And,
    Xor,
    Equal,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}