use crate::TokenPosition;

use super::*;


#[derive(Debug)]
pub struct Method<'a> {
    pub name: &'a str,
    pub name_pos: TokenPosition,
    pub task_name: &'a str,
    pub task_name_pos: TokenPosition,
    pub task_terms: Vec<Symbol<'a>>,
    pub params: Vec<Symbol<'a>>,
    pub precondition: Option<Formula<'a>>,
    pub tn: HTN<'a>
}