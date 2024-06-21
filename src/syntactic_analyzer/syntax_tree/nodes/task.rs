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