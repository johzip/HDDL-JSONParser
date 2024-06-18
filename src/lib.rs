mod lexical_analyzer;
mod syntactic_analyzer;
mod semantic_analyzer;
mod parsing_errors;

use lexical_analyzer::*;

pub fn analyze(program: Vec<u8>) -> Result<(), String>{
    let lexer = LexicalAnalyzer::new(program);
    let parser = syntactic_analyzer::Parser::new(&lexer);
    match parser.parse() {
        Ok(_) => {
            return Ok(());
        },
        _ => panic!("wrong program")
    }
}