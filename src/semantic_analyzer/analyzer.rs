use super::*;

pub fn verify_semantics<'a>(ast: &'a SyntaxTree<'a>) -> Result<(), SemanticError<'a>> {
    let _ = check_duplicate_objects(&ast.objects)?;
    let _ = check_duplicate_requirements(&ast.requirements)?;
    let _ = check_duplicate_predicates(&ast.predicates)?;
    let _ = check_duplicate_actions(&ast.actions)?;
    let _ = check_duplicate_compound_tasks(&ast.compound_tasks)?;
    let _ = check_duplicate_methods(&ast.methods)?;
    
    // assert predicate arg types are declared
    for predicate in ast.predicates.iter() {
        let _ = check_type_declarations(&predicate.variables, &ast.types)?;
    }

    for task in ast.compound_tasks.iter() {
        // assert parameter types are declared
        let _ = check_type_declarations(&task.parameters, &ast.types)?;
    }
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
        // TODO: Assert the compound task is valid
        
        // Assert subtasks are valid
        check_subtask_declarations(&method.tn.subtasks, &ast.compound_tasks, &ast.actions)?;
    }
    Ok(())
}
