use super::*;
pub struct ParseTree {
    root: Vec<TreeNodes>
}

impl ParseTree {
    pub fn new() -> ParseTree {
        ParseTree { root: vec![] }
    }
}