use super::*;

impl<'a> SemanticAnalyzer<'a> {
    pub fn verify_problem(&self, problem: ProblemAST) -> Result<Vec<WarningType>, SemanticErrorType> {
        if let Some(duplicate) = check_duplicate_objects(&problem.objects) {
            return Err(duplicate);
        }
        Ok(vec![])
        // TODO: a lot of problem semantic tests
    }
}
