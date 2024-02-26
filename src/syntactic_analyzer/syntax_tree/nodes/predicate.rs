use super::*;

pub struct Predicate<'a> {
    pub name: &'a str,
    pub variables: TypedList<'a>
}

impl <'a> Predicate<'a> {
    pub fn new(name: &'a str, variables: TypedList<'a>) -> Predicate<'a> {
        Predicate {
            name,
            variables
        }
    }
}