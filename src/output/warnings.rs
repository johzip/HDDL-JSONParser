#[derive(Debug)]
pub enum WarningType<'a> {
    // Action Errors
    UnsatisfiableActionPrecondition(&'a str),
    // TODO: test
    UnsatisfiableMethodPrecondition(&'a str),
    // TODO: implement
    ImmutablePredicate(&'a str),
    // Compound Task errors
    // TODO: implement
    NoPrimitiveRefinement(&'a str),
    // Redundant Elements
    // TODO: implement
    UnusedType(&'a str),
    // TODO: implement
    UnusedPredicate(&'a str),
    // TODO: implement
    UnusedParameter(&'a str),
    // TODO: implement
    RedundantEffect
}

impl <'a> std::fmt::Display for WarningType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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