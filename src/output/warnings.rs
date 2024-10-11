#[derive(Debug)]
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
        // ANSI escape code for yellow
        let yellow = "\x1b[33m";
        // ANSI escape code to reset text color
        let reset = "\x1b[0m";
        write!(f, "{}Warning: {}", yellow, reset)?;
        match self {
            Self::UnsatisfiableActionPrecondition(action) => {
                writeln!(f, "The precondition of action {} cannot be satisfied", action)
            }
            Self::UnsatisfiableMethodPrecondition(method) => {
                writeln!(f, "The precondition of method {} cannot be satisfied", method)
            }
            Self::ImmutablePredicate(predicate) => {
                writeln!(f, "Predicate {} does not appear in the effect of any action", predicate)
            }
            Self::NoPrimitiveRefinement(task) => {
                writeln!(f, "Compound task {} does not have a primitive refinement", task)
            }
            Self::UnusedType(type_name) => {
                writeln!(f, "Type {} is declared, but never used", type_name)
            }
            Self::UnusedPredicate(predicate) => {
                writeln!(f, "Predicate {} is declared, but never used", predicate)
            }
            Self::UnusedParameter(parameter) => {
                writeln!(f, "Parameter {} is declared, but never used", parameter)
            }
            Self::RedundantEffect => {
                // TODO:
                todo!()
            }
        }
    }
}