use std::hash::Hash;

use crate::TokenPosition;

use super::*;

#[derive(Clone, Debug)]
pub struct Predicate<'a> {
    pub name: &'a str,
    pub name_pos: TokenPosition,
    pub variables: Vec<Symbol<'a>>
}

impl <'a> Predicate<'a> {
    pub fn new(name: &'a str, name_pos: TokenPosition, variables: Vec<Symbol<'a>>) -> Predicate<'a> {
        Predicate {
            name,
            name_pos,
            variables
        }
    }
    pub fn new_dummy(name: &'a str) -> Predicate {
        Predicate {
            name,
            name_pos: TokenPosition { line: 0, column: (0, None) },
            variables: vec![]
        }
    }
}

impl <'a> PartialEq for Predicate<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl <'a> Eq for Predicate<'a> {}

impl <'a> Hash for Predicate<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}