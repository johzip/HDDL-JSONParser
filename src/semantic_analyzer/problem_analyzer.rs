use super::*;
use std::collections::{HashMap, HashSet};

impl<'a> SemanticAnalyzer<'a> {
    pub fn verify_problem(
        &self,
        problem: ProblemAST,
    ) -> Result<Vec<WarningType>, SemanticErrorType> {
        // TODO: test
        if let Some(error) = self.type_checker.check_type_declarations(&problem.objects) {
            return Err(error);
        }

        // check for duplicate objects
        let mut object_types = HashMap::new();
        for obj in problem.objects {
            if object_types.contains_key(obj.name) {
                return Err(SemanticErrorType::DuplicateObjectDeclaration(
                    obj.name.to_string(),
                ));
            } else {
                object_types.insert(obj.name, obj.symbol_type);
            }
        }

        // check initial task network ordering is acyclic
        if let Some(htn) = problem.init_tn {
            if !htn.tn.orderings.is_acyclic() {
                return Err(SemanticErrorType::CyclicOrderingDeclaration);
            }
        }



        Ok(vec![])
        // TODO: a lot of problem semantic tests
    }
}
