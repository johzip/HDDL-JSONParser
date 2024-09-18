mod lexical_analyzer;
mod syntactic_analyzer;
mod semantic_analyzer;
mod output;

use crate::lexical_analyzer::TokenPosition;
use lexical_analyzer::LexicalAnalyzer;

pub struct HDDLAnalyzer {}

impl HDDLAnalyzer {
    pub fn verify(program: &Vec<u8>) -> Result<(), String> {
        let lexer = LexicalAnalyzer::new(&program);
        let parser = syntactic_analyzer::Parser::new(lexer);
        match parser.parse() {
            Ok(ast) => {
                let semantic_verifier = semantic_analyzer::SemanticAnalyzer::new(&ast);
                match semantic_verifier.verify_ast() {
                    Ok(_) => {Ok(())}
                    Err(semantic_error) => Err(semantic_error.to_string())
                }
            }
            Err(parsing_error) => Err(parsing_error.to_string())
        }
    }
}
