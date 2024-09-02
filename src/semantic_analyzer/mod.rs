mod analyzer;
mod duplicate_detection;
mod undefined_elements;
mod cycle_detection;
mod tests;

use crate::syntactic_analyzer::*;
use crate::parsing_errors::*;
use duplicate_detection::*;
use undefined_elements::*;

extern crate petgraph;