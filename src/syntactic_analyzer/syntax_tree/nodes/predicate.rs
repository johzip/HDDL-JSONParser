use super::*;

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