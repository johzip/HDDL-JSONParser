use std::collections::HashSet;

use super::*;
use crate::lexical_analyzer::RequirementType;

pub struct DomainSemanticAnalyzer<'a> {
    domain: &'a DomainAST<'a>,
    pub type_checker: DomainTypeChecker<'a>,
}

impl<'a> DomainSemanticAnalyzer<'a> {
    pub fn new(domain: &'a DomainAST<'a>) -> DomainSemanticAnalyzer<'a> {
        DomainSemanticAnalyzer {
            domain,
            type_checker: DomainTypeChecker::new(&domain.types),
        }
    }

    pub fn verify_domain(&'a self) -> Result<SymbolTable<'a>, SemanticErrorType> {
        // Assert there are no duplicate requirements
        if let Some(duplicate) =
            DomainSemanticAnalyzer::check_duplicate_requirements(&self.domain.requirements)
        {
            return Err(duplicate);
        }
        // Assert type hierarchy is acyclic
        let _ = self.type_checker.verify_type_hierarchy()?;
        let mut warnings = vec![];
        // Domain declarations
        let declared_predicates = self.verify_predicates()?;
        let declared_tasks = self.verify_compound_tasks()?;
        let mut declared_constants = HashSet::new();
        match &self.domain.constants {
            Some(constants) => {
                for c in constants {
                    declared_constants.insert(c);
                }
            }
            None => {}
        }

        // assert actions are correct
        let mut declared_actions = HashSet::new();
        for action in self.domain.actions.iter() {
            if !declared_actions.insert(action) {
                return Err(SemanticErrorType::DuplicateActionDeclaration(
                    action.name.to_string(),
                ));
            }
            // assert precondition predicates are declared
            match &action.preconditions {
                Some(precondition) => {
                    check_predicate_declarations(precondition, &self.domain.predicates)?;
                    let precond_predicates = precondition.get_propositional_predicates();
                    self.type_checker.check_formula(
                        &precond_predicates,
                        &action.parameters,
                        &declared_constants,
                        &declared_predicates,
                    )?;
                    if !precondition.is_sat() {
                        warnings.push(WarningType::UnsatisfiableActionPrecondition(
                            action.name.to_string(),
                        ));
                    }
                }
                _ => {}
            }
            // assert effect predicates are declared
            match &action.effects {
                Some(effect) => {
                    check_predicate_declarations(effect, &self.domain.predicates)?;
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
        for method in self.domain.methods.iter() {
            if !declared_methods.insert(method.name) {
                return Err(SemanticErrorType::DuplicateMethodDeclaration(
                    method.name.to_string(),
                ));
            }
            // Assert preconditions are valid
            match &method.precondition {
                Some(precondition) => {
                    check_predicate_declarations(precondition, &self.domain.predicates)?;
                    let precond_predicates = precondition.get_propositional_predicates();
                    self.type_checker.check_formula(
                        &precond_predicates,
                        &method.params,
                        &declared_constants,
                        &declared_predicates,
                    )?;
                    if !precondition.is_sat() {
                        warnings.push(WarningType::UnsatisfiableMethodPrecondition(
                            method.name.to_string(),
                        ));
                    }
                }
                _ => {}
            }
            // Assert task is defined
            if !declared_tasks.contains(method.task_name) {
                return Err(SemanticErrorType::UndefinedTask(
                    method.task_name.to_string(),
                ));
            } else {
                // Assert task arity is consistent
                for declared_compound_task in self.domain.compound_tasks.iter() {
                    if method.task_name == declared_compound_task.name {
                        if method.task_terms.len() != declared_compound_task.parameters.len() {
                            return Err(SemanticErrorType::InconsistentTaskArity(
                                method.task_name.to_string(),
                            ));
                        } else {
                            break;
                        }
                    }
                }
            }

            // Assert task type is consistent
            match self.type_checker.is_task_consistent(
                &method.task_name,
                &method.task_terms.iter().map(|x| x.name).collect(),
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
            if !method.tn.orderings.is_acyclic() {
                return Err(SemanticErrorType::CyclicOrderingDeclaration);
            }
        }
        let type_hierarchy = self.type_checker.get_type_hierarchy();
        Ok(SymbolTable {
            warnings: warnings,
            constants: declared_constants,
            predicates: declared_predicates,
            tasks: declared_tasks,
            actions: declared_actions,
            type_hierarchy: type_hierarchy
        })
    }

    // returns declared predicates (if there is no error)
    fn verify_predicates(&'a self) -> Result<HashSet<&'a Predicate>, SemanticErrorType> {
        let mut declared_predicates = HashSet::new();
        for predicate in self.domain.predicates.iter() {
            if !declared_predicates.insert(predicate) {
                return Err(SemanticErrorType::DuplicatePredicateDeclaration(
                    predicate.name.to_string(),
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
    fn verify_compound_tasks(&'a self) -> Result<HashSet<&Task<'a>>, SemanticErrorType> {
        let mut declared_tasks = HashSet::new();
        for task in self.domain.compound_tasks.iter() {
            if !declared_tasks.insert(task) {
                return Err(SemanticErrorType::DuplicateCompoundTaskDeclaration(
                    task.name.to_string(),
                ));
            }
            // assert parameter types are declared
            if let Some(error) = self.type_checker.check_type_declarations(&task.parameters) {
                return Err(error);
            }
        }
        Ok(declared_tasks)
    }

    pub fn check_duplicate_requirements(
        requirements: &'a Vec<RequirementType>,
    ) -> Option<SemanticErrorType> {
        let mut names = HashSet::new();
        for req in requirements {
            if !names.insert(req) {
                return Some(SemanticErrorType::DuplicateRequirementDeclaration(*req));
            }
        }
        None
    }

    // fn verify_formula(formula: &Formula<'a>, declared_predicates: HashSet<u>)
}
