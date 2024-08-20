use super::*;

pub struct Action<'a> {
    pub name: &'a str,
    pub parameters: Vec<Variable<'a>>,
    pub preconditions: Option<Formula<'a>>,
    pub effects: Option<Formula<'a>>
}