#[derive(Debug, Clone)]
pub enum WarningType {
    // Action Errors
    UnsatisfiableActionPrecondition(String),
    // TODO: test
    UnsatisfiableMethodPrecondition(String),
    // TODO: implement
    ImmutablePredicate(String),
    // Compound Task errors
    // TODO: implement
    NoPrimitiveRefinement(String),
    // Redundant Elements
    // TODO: implement
    UnusedType(String),
    // TODO: implement
    UnusedPredicate(String),
    // TODO: implement
    UnusedParameter(String),
    // TODO: implement
    RedundantEffect
}

impl std::fmt::Display for WarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::UnsatisfiableActionPrecondition(action) => {
                write!(f, "The precondition of action {} cannot be satisfied", action)
            }
            Self::UnsatisfiableMethodPrecondition(method) => {
                write!(f, "The precondition of method {} cannot be satisfied", method)
            }
            Self::ImmutablePredicate(predicate) => {
                write!(f, "Predicate {} does not appear in the effect of any action", predicate)
            }
            Self::NoPrimitiveRefinement(task) => {
                write!(f, "Compound task {} does not have a primitive refinement", task)
            }
            Self::UnusedType(type_name) => {
                write!(f, "Type {} is declared, but never used", type_name)
            }
            Self::UnusedPredicate(predicate) => {
                write!(f, "Predicate {} is declared, but never used", predicate)
            }
            Self::UnusedParameter(parameter) => {
                write!(f, "Parameter {} is declared, but never used", parameter)
            }
            Self::RedundantEffect => {
                // TODO:
                todo!()
            }
        }
    }
}