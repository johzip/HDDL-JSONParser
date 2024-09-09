use std::hash::Hash;

use super::*;

#[derive(Clone, Debug)]
pub struct Predicate<'a> {
    pub name: &'a str,
    pub variables: Vec<Variable<'a>>
}

impl <'a> Predicate<'a> {
    pub fn new(name: &'a str, variables: Vec<Variable<'a>>) -> Predicate<'a> {
        Predicate {
            name,
            variables
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