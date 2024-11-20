use crate::lexical_analyzer::RequirementType;
use std::fmt;

#[derive(Debug)]
pub enum SemanticErrorType{
    // Duplicate Errors
    DuplicateObjectDeclaration(String),
    DuplicateRequirementDeclaration(RequirementType),
    DuplicatePredicateDeclaration(String),
    DuplicateActionDeclaration(String),
    DuplicateCompoundTaskDeclaration(String),
    DuplicateMethodDeclaration(String),
    // Undefined Entities
    UndefinedPredicate(String),
    UndefinedType(String),
    UndefinedSubtask(String),
    UndefinedTask(String),
    UndefinedParameter(String),
    UndefinedObject(String),
    // Inconsistency Error
    InconsistentPredicateArity(ArityError),
    InconsistentTaskArity(ArityError),
    InconsistentPredicateArgType(TypeError),
    InconsistentTaskArgType(TypeError),
    // Ordering Errors
    CyclicTypeDeclaration,
    CyclicOrderingDeclaration,
}

impl fmt::Display for SemanticErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Duplicate Errors
            SemanticErrorType::DuplicateObjectDeclaration(obj) => write!(f, "object {} is declared multiple times.", obj),
            SemanticErrorType::DuplicateRequirementDeclaration(req_type) => write!(f, "requirement {:?} is declared multiple times.", req_type),
            SemanticErrorType::DuplicatePredicateDeclaration(pred) => write!(f, "predicate {} is declared multiple times.", pred),
            SemanticErrorType::DuplicateActionDeclaration(action) => write!(f, "action {} is declared multiple times.", action),
            SemanticErrorType::DuplicateCompoundTaskDeclaration(task) => write!(f, "compound task {} is declared multiple times.", task),
            SemanticErrorType::DuplicateMethodDeclaration(method) => write!(f, "method {} is declared multiple times.", method),
            // Undefined Entities
            SemanticErrorType::UndefinedPredicate(pred) => write!(f, "predicate {} is not defined.", pred),
            SemanticErrorType::UndefinedType(typ) => write!(f, "type {} is not defined.", typ),
            SemanticErrorType::UndefinedSubtask(subtask) => write!(f, "subtask {} is not defined.", subtask),
            SemanticErrorType::UndefinedTask(task) => write!(f, "task {} is not defined.", task),
            SemanticErrorType::UndefinedParameter(param) => write!(f, "parameter {} is not defined.", param),
            SemanticErrorType::UndefinedObject(object) => write!(f, "object {} is not defined.", object),
            // Inconsistency Error
            SemanticErrorType::InconsistentPredicateArity(ar_error) => {
                write!(f, "Predicate {} takes {} parameters, but {} are given.", ar_error.symbol, ar_error.expected_arity, ar_error.found_arity)
            }
            SemanticErrorType::InconsistentTaskArity(ar_error) => {
                write!(f, "Task {} takes {} parameters, but {} are given.", ar_error.symbol, ar_error.expected_arity, ar_error.found_arity)
            }
            SemanticErrorType::InconsistentPredicateArgType(type_error) => write!(f, "{}", type_error),
            SemanticErrorType::InconsistentTaskArgType(type_error) => write!(f, "{}", type_error),
            // Ordering Errors
            SemanticErrorType::CyclicTypeDeclaration => write!(f, "Cyclic type declaration"),
            SemanticErrorType::CyclicOrderingDeclaration => write!(f, "Cyclic ordering declaration"),
        }
    }
}


#[derive(Debug)]
pub struct TypeError {
    pub expected: Option<String>,
    pub found: Option<String>,
    pub var_name: String
} 

impl fmt::Display for TypeError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type error for variable {}. ", self.var_name)?;
        match (&self.expected, &self.found) {
            (Some(expected), Some(found)) => {
                write!(f, "Expected object of type '{}', but found '{}'.", expected, found)
            },
            (Some(expected), None) => {
                write!(f, "Expected object of type '{}', but did not find any typing.", expected)
            },
            (None, Some(found)) => {
                write!(f, "Expected no type, but found '{}'.", found)
            },
            (None, None) => {
                unreachable!()
            },
        }
    }
}

#[derive(Debug)]
pub struct ArityError {
    pub symbol: String,
    pub expected_arity: u32,
    pub found_arity: u32
} 