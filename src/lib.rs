mod lexical_analyzer;
mod syntactic_analyzer;
mod semantic_analyzer;
mod output;

use crate::lexical_analyzer::TokenPosition;
use lexical_analyzer::LexicalAnalyzer;
use syntactic_analyzer::AbstractSyntaxTree;

pub struct HDDLAnalyzer {}

impl HDDLAnalyzer {
    pub fn verify(domain: &Vec<u8>, problem: Option<&Vec<u8>>) -> Result<Vec<output::WarningType>, output::ParsingError> {
        let lexer = LexicalAnalyzer::new(&domain);
        let domain_parser = syntactic_analyzer::Parser::new(lexer);
        let domain_ast = domain_parser.parse()?;
        if let AbstractSyntaxTree::Domain(d) = domain_ast {
            let semantic_verifier = semantic_analyzer::SemanticAnalyzer::new(&d);
            let warnings = semantic_verifier.verify_domain()?;
            match problem {
                Some(p) => {
                    let lexer = LexicalAnalyzer::new(p);
                    let problem_parser = syntactic_analyzer::Parser::new(lexer);
                    let problem_ast = problem_parser.parse()?;
                    if let AbstractSyntaxTree::Problem(p_ast) = problem_ast {
                        Ok(semantic_verifier.verify_problem(p_ast)?)
                    } else {
                        panic!("expected problem, found domain")
                    }
                },
                None => Ok(warnings)
            }
        } else {
            panic!("expected domain, found problem")
        }
    }
}
