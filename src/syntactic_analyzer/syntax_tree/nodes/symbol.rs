use std::{borrow::Borrow, hash::Hash};

#[derive(Clone, Debug)]
pub struct Symbol<'a> {
    pub name: &'a str,
    pub symbol_type: Option<&'a str>
}

impl <'a> Symbol<'a> {
    pub fn new(name: &'a str, var_type: Option<&'a str>) -> Symbol<'a> {
        Symbol {
            name,
            symbol_type: var_type
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