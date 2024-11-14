use super::*;
use std::collections::HashMap;

pub struct ProblemSemanticAnalyzer<'a> {
    problem: &'a ProblemAST<'a>,
    type_checker: ProblemTypeChecker<'a>
}

impl<'a> ProblemSemanticAnalyzer<'a> {
    pub fn new(
        problem: &'a ProblemAST<'a>,
        domain_symbols: SymbolTable<'a>,
    ) -> ProblemSemanticAnalyzer<'a> {
        ProblemSemanticAnalyzer {
            problem,
            type_checker: ProblemTypeChecker::new(
                domain_symbols
            ),
        }
    }

    pub fn verify_problem(
        &self,
    ) -> Result<Vec<WarningType>, SemanticErrorType> {
        // TODO: test
        if let Some(error) = self.type_checker.check_type_declarations(&self.problem.objects) {
            return Err(error);
        }

        // check for duplicate objects
        let mut object_types = HashMap::new();
        for obj in self.problem.objects.iter() {
            if object_types.contains_key(obj.name) {
                return Err(SemanticErrorType::DuplicateObjectDeclaration(
                    obj.name.to_string(),
                ));
            } else {
                object_types.insert(obj.name, obj.symbol_type);
            }
        }

        // check initial task network ordering is acyclic
        if let Some(htn) = &self.problem.init_tn {
            if !htn.tn.orderings.is_acyclic() {
                return Err(SemanticErrorType::CyclicOrderingDeclaration);
            }
        }

        Ok(self.type_checker.symbol_table.warnings.iter().cloned().collect())
        // TODO: a lot of problem semantic tests
    }
}
