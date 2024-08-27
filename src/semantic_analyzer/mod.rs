mod semantic_errors;
mod analyzer;
mod duplicate_detection;
mod undefined_elements;
mod tests;

use crate::syntactic_analyzer::*;
pub use semantic_errors::*;
use duplicate_detection::*;
use undefined_elements::*;