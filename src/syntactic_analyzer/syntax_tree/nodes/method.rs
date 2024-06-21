use super::*;

pub struct Method<'a> {
    pub name:  &'a str,
    pub task_name: &'a str,
    pub task_terms: Arguements<'a>,
    pub params: Arguements<'a>,
    pub precondition: Option<Formula<'a>>,
    pub tn: HTN<'a>
}