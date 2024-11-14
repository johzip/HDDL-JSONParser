use super::*;

pub struct ProblemTypeChecker<'a> {
    generic_type_checker: TypeChecker<'a>,
    pub symbol_table: SymbolTable<'a>
}

impl<'a> ProblemTypeChecker<'a> {
    pub fn new(
        symbol_table: SymbolTable<'a>
    ) -> ProblemTypeChecker<'a> {
        ProblemTypeChecker {
            generic_type_checker: TypeChecker { type_hierarchy: symbol_table.type_hierarchy.clone() },
            symbol_table
        }
    }
    pub fn check_type_declarations(&self,
        parameters: &Vec<Symbol<'a>>,
    ) -> Option<SemanticErrorType> {
        self.generic_type_checker.check_type_declarations(parameters)
    }
}
