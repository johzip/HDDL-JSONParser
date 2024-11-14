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
                domain_symbols,
                problem
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

        // TODO: test
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

        // TODO: test
        // check the consistency of init predicates
        for predicate in self.problem.init_state.iter() {
            let _ = self.type_checker.check_predicate_instantiation(predicate)?;
        }

        // TODO: test
        // check the initial task network
        if let Some(htn) = &self.problem.init_tn {
            if !htn.tn.orderings.is_acyclic() {
                return Err(SemanticErrorType::CyclicOrderingDeclaration);
            }

            for subtask in htn.tn.subtasks.iter() {
                let _ = self.type_checker.check_subtask_instantiation(subtask)?;
            }
        }

        Ok(self.type_checker.symbol_table.warnings.iter().cloned().collect())
    }
}
