mod table;

pub use table::SymbolTable;
use crate::semantic_analyzer::{SemanticError, SemanticErrorType};
use crate::lexical_analyzer::RequirementType;