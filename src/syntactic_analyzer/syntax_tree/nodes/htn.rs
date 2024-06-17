use super::*;

pub struct InitialTaskNetwork<'a> {
    pub parameters: Option<Arguements<'a>>,
    pub tn: HTN<'a>
}

pub struct HTN<'a> {
    pub subtasks: Vec<Subtask<'a>>,
    pub orderings: TaskOrdering<'a>,
    pub constraints: Option<Vec<Constraint<'a>>>, 
}

pub struct Subtask<'a> {
    pub id: Option<&'a str>,
    pub task_symbol: &'a str,
    pub terms: Vec<&'a str>
}

pub enum Constraint<'a> {
    Equal(&'a str, &'a str),
    NotEqual(&'a str, &'a str)
}

pub enum TaskOrdering<'a> {
    Total,
    Partial(Vec<(&'a str, &'a str)>)
}