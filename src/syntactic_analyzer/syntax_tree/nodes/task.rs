use super::*;

pub struct Task<'a> {
    pub name: &'a str,
    pub parameters: TypedList<'a>
}

impl <'a> Task <'a> {
    pub fn new(name: &'a str, parameters: TypedList<'a>) -> Task<'a> {
        Task {
            name,
            parameters
        }
    }
}