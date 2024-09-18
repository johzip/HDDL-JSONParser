use crate::TokenPosition;

use super::*;


#[derive(Debug)]
pub struct InitialTaskNetwork<'a> {
    pub parameters: Option<Vec<Symbol<'a>>>,
    pub tn: HTN<'a>
}

#[derive(Debug)]
pub struct HTN<'a> {
    pub subtasks: Vec<Subtask<'a>>,
    pub orderings: TaskOrdering<'a>,
    pub constraints: Option<Vec<Constraint<'a>>>, 
}

#[derive(Debug)]
pub struct Subtask<'a> {
    pub id: Option<&'a str>,
    pub id_pos: Option<TokenPosition>,
    pub task_symbol: &'a str,
    pub task_symbol_pos: TokenPosition,
    pub terms: Vec<&'a str>,
    pub terms_pos: Vec<TokenPosition>,
}


#[derive(Debug)]
pub enum Constraint<'a> {
    Equal(&'a str, &'a str),
    NotEqual(&'a str, &'a str)
}

#[derive(Debug)]
pub enum TaskOrdering<'a> {
    Total,
    Partial(Vec<(&'a str, &'a str)>)
}