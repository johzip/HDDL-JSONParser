mod lexical_errors;
mod syntactical_errors;
mod generic_error;
mod semantic_errors;
mod warnings;

pub use lexical_errors::*;
pub use syntactical_errors::*;
pub use generic_error::*;
pub use semantic_errors::*;
pub use warnings::*;

use crate::lexical_analyzer::TokenType;