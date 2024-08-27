use super::*;

pub fn verify_semantics<'a>(ast: &'a SyntaxTree<'a>) -> Result<(), SemanticError<'a>> {
    let _ = check_duplicate_objects(&ast.objects)?;
    let _ = check_duplicate_requirements(&ast.requirements)?;
    let _ = check_duplicate_predicates(&ast.predicates)?;
    let _ = check_duplicate_actions(&ast.actions)?;
    let _ = check_duplicate_compound_tasks(&ast.compound_tasks)?;
    let _ = check_duplicate_methods(&ast.methods)?;
    for action in ast.actions.iter() {
        // assert parameter types are declared
        let _ = check_type_declarations(&action.parameters, &ast.types)?;
        // assert precondition predicates are declared
        match &action.preconditions {
            Some(precondition) => {
                check_predicate_declarations(precondition, &ast.predicates)?;
            }
            _ => {}
        }
        // assert effect predicates are declared
        match &action.effects {
            Some(effect) => {
                check_predicate_declarations(effect, &ast.predicates)?;
            }
            _ => {}
        }
    }
    for method in ast.methods.iter() {
        // assert parameter types are declared
        let _ = check_type_declarations(&method.params, &ast.types)?;
        // Assert preconditions are valid
        match &method.precondition {
            Some(precondition) => {
                check_predicate_declarations(precondition, &ast.predicates)?;
            }
            _ => {}
        }
        // Assert subtasks are valid
        check_subtask_declarations(&method.tn, &ast.compound_tasks, &ast.actions)?;
    }
    Ok(())
}
