use super::*;

pub struct Method<'a> {
    pub name:  &'a str,
    pub task_name: &'a str,
    pub task_terms: Vec<Variable<'a>>,
    pub params: Vec<Variable<'a>>,
    pub precondition: Option<Formula<'a>>,
    pub tn: HTN<'a>
}