use std::collections::HashSet;

use cycle_detection::check_ordering_acyclic;

use super::*;

pub struct SemanticAnalyzer<'a> {
    ast: &'a SyntaxTree<'a>,
    type_checker: TypeChecker<'a>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(ast: &'a SyntaxTree<'a>) -> SemanticAnalyzer<'a> {
        SemanticAnalyzer {
            ast,
            type_checker: TypeChecker::new(&ast.types),
        }
    }

    pub fn verify_ast(&'a self) -> Result<Vec<Warning<'a>>, SemanticErrorType<'a>> {
        // Assert there are no duplicate objects
        if let Some(duplicate) = check_duplicate_objects(&self.ast.objects) {
            return Err(duplicate);
        // Assert there are no duplicate requirements
        } else if let Some(duplicate) = check_duplicate_requirements(&self.ast.requirements) {
            return Err(duplicate);
        // Assert type hierarchy is acyclic
        } else if let Some(cycle) = self.type_checker.check_acyclicity() {
            return Err(cycle);
        }
        let mut warnings = vec![];
        // Domain declarations
        let declared_predicates = self.verify_predicates()?;
        let declared_tasks = self.verify_compound_tasks()?;
        let mut declared_constants = HashSet::new();
        match &self.ast.constants {
            Some(constants) => {
                for c in constants {
                    declared_constants.insert(c);
                }
            },
            None => {}
        }

        // assert actions are correct
        let mut declared_actions = HashSet::new();
        for action in self.ast.actions.iter() {
            if !declared_actions.insert(action) {
                return Err(SemanticErrorType::DuplicateActionDeclaration(action.name));
            }
            // assert precondition predicates are declared
            match &action.preconditions {
                Some(precondition) => {
                    check_predicate_declarations(precondition, &self.ast.predicates)?;
                    let precond_predicates = precondition.get_propositional_predicates();
                    self.type_checker.check_formula(
                        &precond_predicates,
                        &action.parameters,
                        &declared_constants,
                        &declared_predicates,
                    )?;
                    if !precondition.is_sat() {
                        warnings.push(Warning::UnsatisfiableActionPrecondition(&action.name));
                    }
                }
                _ => {}
            }
            // assert effect predicates are declared
            match &action.effects {
                Some(effect) => {
                    check_predicate_declarations(effect, &self.ast.predicates)?;
                    let eff_predicates = effect.get_propositional_predicates();
                    self.type_checker.check_formula(
                        &eff_predicates,
                        &action.parameters,
                        &declared_constants,
                        &declared_predicates,
                    )?;
                }
                _ => {}
            }
        }

        // assert methods are correct
        let mut declared_methods = HashSet::new();
        for method in self.ast.methods.iter() {
            if !declared_methods.insert(method.name) {
                return Err(SemanticErrorType::DuplicateMethodDeclaration(method.name));
            }
            // Assert preconditions are valid
            match &method.precondition {
                Some(precondition) => {
                    check_predicate_declarations(precondition, &self.ast.predicates)?;
                    let precond_predicates = precondition.get_propositional_predicates();
                    self.type_checker.check_formula(
                        &precond_predicates,
                        &method.params,
                        &declared_constants,
                        &declared_predicates,
                    )?;
                    if !precondition.is_sat() {
                        warnings.push(Warning::UnsatisfiableMethodPrecondition(&method.name));
                    }
                }
                _ => {}
            }
            // Assert task is defined
            if !declared_tasks.contains(method.task_name) {
                return Err(SemanticErrorType::UndefinedTask(&method.task_name));
            } else {
                // Assert task arity is consistent
                for declared_compound_task in self.ast.compound_tasks.iter() {
                    if method.task_name == declared_compound_task.name {
                        if method.task_terms.len() != declared_compound_task.parameters.len() {
                            return Err(SemanticErrorType::InconsistentTaskArity(&method.task_name));
                        } else {
                            break;
                        }
                    }
                }
            }

            // Assert task type is consistent
            match self.type_checker.is_task_consistent(
                &method.task_name,
                &method.task_terms.iter().map(|x| {
                    x.name
                }).collect(),
                &method.params,
                &declared_constants,
                &declared_tasks,
                &HashSet::new(),
            ) {
                Err(SemanticErrorType::UndefinedSubtask(task_name)) => {
                    return Err(SemanticErrorType::UndefinedTask(task_name));
                }
                _ => {}
            }

            // Assert subtask types are consistent
            for subtask in method.tn.subtasks.iter() {
                let _ = self.type_checker.is_task_consistent(
                    &subtask.task_symbol,
                    &subtask.terms,
                    &method.params,
                    &declared_constants,
                    &declared_tasks,
                    &declared_actions,
                )?;
            }
            // Assert orderings are acyclic
            check_ordering_acyclic(&method.tn)?;
        }
        Ok(warnings)
    }

    // returns declared predicates (if there is no error)
    fn verify_predicates(&'a self) -> Result<HashSet<&'a Predicate>, SemanticErrorType<'a>> {
        let mut declared_predicates = HashSet::new();
        for predicate in self.ast.predicates.iter() {
            if !declared_predicates.insert(predicate) {
                return Err(SemanticErrorType::DuplicatePredicateDeclaration(
                    &predicate.name,
                ));
            }
            if let Some(error) = self
                .type_checker
                .check_type_declarations(&predicate.variables)
            {
                return Err(error);
            }
        }
        Ok(declared_predicates)
    }

    // returns declared compound tasks (if there is no error)
    fn verify_compound_tasks(&'a self) -> Result<HashSet<&Task<'a>>, SemanticErrorType<'a>> {
        let mut declared_tasks = HashSet::new();
        for task in self.ast.compound_tasks.iter() {
            if !declared_tasks.insert(task) {
                return Err(SemanticErrorType::DuplicateCompoundTaskDeclaration(task.name));
            }
            // assert parameter types are declared
            if let Some(error) = self.type_checker.check_type_declarations(&task.parameters) {
                return Err(error);
            }
        }
        Ok(declared_tasks)
    }

    // fn verify_formula(formula: &Formula<'a>, declared_predicates: HashSet<u>)
}
