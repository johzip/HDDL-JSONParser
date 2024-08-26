use super::*;
pub struct SemanticAnalyzer<'a> {
    pub ast: SyntaxTree<'a>,
}

impl <'a> SemanticAnalyzer<'a> {
    pub fn new(ast: SyntaxTree<'a>) -> SemanticAnalyzer {
        SemanticAnalyzer { ast }
    }
}