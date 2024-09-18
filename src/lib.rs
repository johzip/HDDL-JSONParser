mod lexical_analyzer;
mod syntactic_analyzer;
mod semantic_analyzer;
mod output;

use lexical_analyzer::*;

pub fn analyze(program: &Vec<u8>) -> Result<(), String> {
    let lexer = LexicalAnalyzer::new(program);
    let parser = syntactic_analyzer::Parser::new(lexer);
    match parser.parse() {
        Ok(ast) => {
            let semantic_verifier = semantic_analyzer::SemanticAnalyzer::new(&ast);
            match semantic_verifier.verify_ast(){
                Ok(_) => {
                    return Ok(());
                },
                token => panic!("{:?}", token)
            }
        }
        token => panic!("wrong program {:?}", token)
    }
    
}