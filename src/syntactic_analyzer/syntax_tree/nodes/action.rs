use std::hash::Hash;

use super::*;

pub struct Action<'a> {
    pub name: &'a str,
    pub parameters: Vec<Symbol<'a>>,
    pub preconditions: Option<Formula<'a>>,
    pub effects: Option<Formula<'a>>
}

impl <'a> Hash for Action<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl <'a> PartialEq for Action<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl <'a> Eq for Action<'a> {}