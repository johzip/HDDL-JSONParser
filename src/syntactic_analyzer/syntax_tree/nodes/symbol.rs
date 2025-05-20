use std::{borrow::Borrow, hash::Hash};

use serde::Serialize;

use crate::TokenPosition;

#[derive(Clone, Debug, Serialize)]
pub struct Symbol<'a> {
    pub name: &'a str,
    pub name_pos: TokenPosition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_pos: Option<TokenPosition>
}

impl <'a> Symbol<'a> {
    pub fn new(name: &'a str, name_pos: TokenPosition, symbol_type: Option<&'a str>, type_pos: Option<TokenPosition>) -> Symbol<'a> {
        Symbol {
            name,
            name_pos,
            symbol_type,
            type_pos
        }
    }
}

impl <'a> Eq for Symbol<'a> {}

impl <'a> PartialEq for Symbol<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl <'a> Hash for Symbol<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl <'a> Borrow<&'a str> for &Symbol<'a> {
    fn borrow(&self) -> &&'a str {
        &self.name
    }
}