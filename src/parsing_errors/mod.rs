mod lexical_errors;
mod syntactical_errors;
mod generic_error;
mod semantic_errors;

pub use lexical_errors::*;
pub use syntactical_errors::*;
pub use generic_error::*;
pub use semantic_errors::*;

use crate::lexical_analyzer::TokenType;