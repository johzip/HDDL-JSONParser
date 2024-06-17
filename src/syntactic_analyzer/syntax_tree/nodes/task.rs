use super::*;

pub struct Task<'a> {
    pub name: &'a str,
    pub parameters: Arguements<'a>
}

impl <'a> Task <'a> {
    pub fn new(name: &'a str, parameters: Arguements<'a>) -> Task<'a> {
        Task {
            name,
            parameters
        }
    }
}