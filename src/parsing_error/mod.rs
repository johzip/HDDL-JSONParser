mod lexical_errors;
mod syntactical_errors;

pub use lexical_errors::*;
pub use syntactical_errors::*;

use crate::lexical_analyzer::Token;