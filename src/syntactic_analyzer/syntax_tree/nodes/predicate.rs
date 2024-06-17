use super::*;

pub struct Predicate<'a> {
    pub name: &'a str,
    pub variables: Arguements<'a>
}

impl <'a> Predicate<'a> {
    pub fn new(name: &'a str, variables: Arguements<'a>) -> Predicate<'a> {
        Predicate {
            name,
            variables
        }
    }
}