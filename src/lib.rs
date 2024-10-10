mod lexical_analyzer;
mod syntactic_analyzer;
mod semantic_analyzer;
mod output;

use crate::lexical_analyzer::TokenPosition;
use lexical_analyzer::LexicalAnalyzer;

pub struct HDDLAnalyzer {}

impl HDDLAnalyzer {
    pub fn verify(program: &Vec<u8>) -> Result<Vec<output::WarningType>, output::ParsingError> {
        let lexer = LexicalAnalyzer::new(&program);
        let parser = syntactic_analyzer::Parser::new(lexer);
        let ast = parser.parse()?;
        let semantic_verifier = semantic_analyzer::SemanticAnalyzer::new(&ast);
        Ok(semantic_verifier.verify_ast()?)
    }
}
