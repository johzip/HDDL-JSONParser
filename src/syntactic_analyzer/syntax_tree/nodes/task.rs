use std::{borrow::Borrow, hash::Hash};

use super::*;

pub struct Task<'a> {
    pub name: &'a str,
    pub parameters: Vec<Symbol<'a>>
}

impl <'a> Task <'a> {
    pub fn new(name: &'a str, parameters: Vec<Symbol<'a>>) -> Task<'a> {
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

impl <'a> Borrow<str> for &Task<'a> {
    fn borrow(&self) -> &'a str {
        &self.name
    }
}