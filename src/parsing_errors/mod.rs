mod lexical_errors;
mod syntactical_errors;
mod generic_error;

pub use lexical_errors::*;
pub use syntactical_errors::*;
pub use generic_error::*;

use crate::lexical_analyzer::TokenType;