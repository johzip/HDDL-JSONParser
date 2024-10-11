mod lexical_analyzer;
mod syntactic_analyzer;
mod semantic_analyzer;
mod output;

use crate::lexical_analyzer::TokenPosition;
use lexical_analyzer::LexicalAnalyzer;
use syntactic_analyzer::AbstractSyntaxTree;

pub struct HDDLAnalyzer {}

impl HDDLAnalyzer {
    pub fn verify_domain(domain: &Vec<u8>) -> Result<Vec<output::WarningType>, output::ParsingError> {
        let lexer = LexicalAnalyzer::new(&domain);
        let parser = syntactic_analyzer::Parser::new(lexer);
        let ast = parser.parse()?;
        if let AbstractSyntaxTree::Domain(d) = ast {
            let semantic_verifier = semantic_analyzer::SemanticAnalyzer::new(&d);
            Ok(semantic_verifier.verify_ast()?)
        } else {
            panic!("expected domain, found problem")
        }
    }
}
