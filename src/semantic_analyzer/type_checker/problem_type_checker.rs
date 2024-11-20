use std::collections::HashMap;

use super::*;

pub struct ProblemTypeChecker<'a> {
    generic_type_checker: TypeChecker<'a>,
    pub symbol_table: SymbolTable<'a>,
    objects: HashMap<&'a str, Option<&'a str>>,
}

impl<'a> ProblemTypeChecker<'a> {
    pub fn new(
        symbol_table: SymbolTable<'a>,
        problem: &'a ProblemAST<'a>,
    ) -> ProblemTypeChecker<'a> {
        let mut objects = HashMap::new();
        for object in problem.objects.iter() {
            objects.insert(object.name, object.symbol_type);
        }
        ProblemTypeChecker {
            generic_type_checker: TypeChecker {
                type_hierarchy: symbol_table.type_hierarchy.clone(),
            },
            symbol_table,
            objects,
        }
    }
    pub fn check_type_declarations(
        &self,
        parameters: &Vec<Symbol<'a>>,
    ) -> Option<SemanticErrorType> {
        self.generic_type_checker
            .check_type_declarations(parameters)
    }

    pub fn check_predicate_instantiation(
        &self,
        predicate: &'a Predicate<'a>,
    ) -> Result<(), SemanticErrorType> {
        let declared_predicates = &self.symbol_table.predicates;
        match declared_predicates.get(predicate) {
            Some(definition) => {
                if definition.variables.len() != predicate.variables.len() {
                    return Err(SemanticErrorType::InconsistentPredicateArity(ArityError {
                        symbol: predicate.name.to_string(),
                        expected_arity: definition.variables.len() as u32,
                        found_arity: predicate.variables.len() as u32,
                    }));
                }
                for (expected, found) in definition.variables.iter().zip(predicate.variables.iter())
                {
                    match self.objects.get(found.name) {
                        Some(object_type) => {
                            let is_consistent = self
                                .generic_type_checker
                                .is_var_type_consistent(*object_type, expected.symbol_type);
                            if !is_consistent {
                                return Err(SemanticErrorType::InconsistentPredicateArgType(
                                    TypeError {
                                        expected: expected.symbol_type.map(String::from),
                                        found: found.symbol_type.map(String::from),
                                        var_name: predicate.name.to_string(),
                                    },
                                ));
                            }
                        }
                        None => {
                            return Err(SemanticErrorType::UndefinedObject(found.name.to_string()));
                        }
                    }
                }
                return Ok(());
            }
            None => {
                return Err(SemanticErrorType::UndefinedPredicate(
                    predicate.name.to_string(),
                ));
            }
        }
    }

    pub fn check_subtask_instantiation(
        &self,
        subtask: &'a Subtask<'a>,
    ) -> Result<(), SemanticErrorType> {
        if self.symbol_table.actions.contains(&subtask.task_symbol) {
            let action = self.symbol_table.actions.get(&subtask.task_symbol).unwrap();
            if action.parameters.len() != subtask.terms.len() {
                return Err(SemanticErrorType::InconsistentTaskArity(ArityError {
                    symbol: subtask.task_symbol.to_string(),
                    expected_arity: action.parameters.len() as u32,
                    found_arity: subtask.terms.len() as u32,
                }));
            }
            for (expected, found) in action.parameters.iter().zip(subtask.terms.iter()) {
                match self.objects.get(found) {
                    Some(object_type) => {
                        let is_consistent = self
                            .generic_type_checker
                            .is_var_type_consistent(*object_type, expected.symbol_type);
                        if !is_consistent {
                            return Err(SemanticErrorType::InconsistentTaskArgType(TypeError {
                                expected: expected.symbol_type.map(String::from),
                                found: object_type.map(String::from),
                                var_name: subtask.task_symbol.to_string(),
                            }));
                        }
                    }
                    None => {
                        return Err(SemanticErrorType::UndefinedObject(found.to_string()));
                    }
                }
            }
            return Ok(());
        } else if self.symbol_table.tasks.contains(subtask.task_symbol) {
            let task = self.symbol_table.tasks.get(&subtask.task_symbol).unwrap();
            if task.parameters.len() != subtask.terms.len() {
                return Err(SemanticErrorType::InconsistentTaskArity(ArityError {
                    symbol: subtask.task_symbol.to_string(),
                    expected_arity: task.parameters.len() as u32,
                    found_arity: subtask.terms.len() as u32,
                }));
            }
            for (expected, found) in task.parameters.iter().zip(subtask.terms.iter()) {
                match self.objects.get(found) {
                    Some(object_type) => {
                        let is_consistent = self
                            .generic_type_checker
                            .is_var_type_consistent(*object_type, expected.symbol_type);
                        if !is_consistent {
                            return Err(SemanticErrorType::InconsistentTaskArgType(TypeError {
                                expected: expected.symbol_type.map(String::from),
                                found: object_type.map(String::from),
                                var_name: subtask.task_symbol.to_string(),
                            }));
                        }
                    }
                    None => {
                        return Err(SemanticErrorType::UndefinedObject(found.to_string()));
                    }
                }
            }
            return Ok(());
        } else {
            return Err(SemanticErrorType::UndefinedSubtask(
                subtask.task_symbol.to_string(),
            ));
        }
    }
}
