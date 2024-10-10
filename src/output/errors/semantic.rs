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
    // Inconsistency Error
    InconsistentPredicateArity(String),
    InconsistentTaskArity(String),
    InconsistentPredicateArgType(TypeError),
    InconsistentTaskArgType(TypeError),
    // Ordering Errors
    CyclicTypeDeclaration(String),
    CyclicOrderingDeclaration(String),
}

impl fmt::Display for SemanticErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Duplicate Errors
            SemanticErrorType::DuplicateObjectDeclaration(obj) => write!(f, "Duplicate object declaration: {}", obj),
            SemanticErrorType::DuplicateRequirementDeclaration(req_type) => write!(f, "Duplicate requirement declaration: {:?}", req_type),
            SemanticErrorType::DuplicatePredicateDeclaration(pred) => write!(f, "Duplicate predicate declaration: {}", pred),
            SemanticErrorType::DuplicateActionDeclaration(action) => write!(f, "Duplicate action declaration: {}", action),
            SemanticErrorType::DuplicateCompoundTaskDeclaration(task) => write!(f, "Duplicate compound task declaration: {}", task),
            SemanticErrorType::DuplicateMethodDeclaration(method) => write!(f, "Duplicate method declaration: {}", method),
            // Undefined Entities
            SemanticErrorType::UndefinedPredicate(pred) => write!(f, "Undefined predicate: {}", pred),
            SemanticErrorType::UndefinedType(typ) => write!(f, "Undefined type: {}", typ),
            SemanticErrorType::UndefinedSubtask(subtask) => write!(f, "Undefined subtask: {}", subtask),
            SemanticErrorType::UndefinedTask(task) => write!(f, "Undefined task: {}", task),
            SemanticErrorType::UndefinedParameter(param) => write!(f, "Undefined parameter: {}", param),
            // Inconsistency Error
            SemanticErrorType::InconsistentPredicateArity(pred) => write!(f, "Inconsistent predicate arity: {}", pred),
            SemanticErrorType::InconsistentTaskArity(task) => write!(f, "Inconsistent task arity: {}", task),
            SemanticErrorType::InconsistentPredicateArgType(type_error) => write!(f, "Inconsistent predicate argument type: {}", type_error),
            SemanticErrorType::InconsistentTaskArgType(type_error) => write!(f, "Inconsistent task argument type: {}", type_error),
            // Ordering Errors
            SemanticErrorType::CyclicTypeDeclaration(typ) => write!(f, "Cyclic type declaration: {}", typ),
            SemanticErrorType::CyclicOrderingDeclaration(order) => write!(f, "Cyclic ordering declaration: {}", order),
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
                write!(f, "\nexpected: {}\n\tfound: {}", expected, found)
            },
            (Some(expected), None) => {
                write!(f, "Expected '{}', but found nothing", expected)
            },
            (None, Some(found)) => {
                write!(f, "Expected no type, but found '{}'", found)
            },
            (None, None) => {
                unreachable!()
            },
        }
    }
}