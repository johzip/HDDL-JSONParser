use std::hash::Hash;

use super::*;

pub struct Task<'a> {
    pub name: &'a str,
    pub parameters: Vec<Variable<'a>>
}

impl <'a> Task <'a> {
    pub fn new(name: &'a str, parameters: Vec<Variable<'a>>) -> Task<'a> {
        Task {
            name,
            parameters
        }
    }
}

impl <'a> Hash for Task<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl <'a> PartialEq for Task<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl <'a> Eq for Task<'a> {}