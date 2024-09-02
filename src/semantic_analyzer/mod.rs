mod analyzer;
mod duplicate_detection;
mod undefined_elements;
mod tests;

use crate::syntactic_analyzer::*;
use crate::parsing_errors::*;
use duplicate_detection::*;
use undefined_elements::*;