use super::*;

pub struct Method<'a> {
    pub name:  &'a str,
    pub task_name: &'a str,
    pub task_terms: Arguements<'a>,
    pub params: Arguements<'a>,
    // TODO:
    // pub precondition: Option<BooleanFormula<'a>>,
    pub tn: HTN<'a>
}