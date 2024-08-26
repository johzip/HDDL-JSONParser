use super::*;

pub fn verify_semantics<'a>(ast: &'a SyntaxTree<'a>) -> Result<(), SemanticError<'a>> {
    let _ = check_duplicate_objects(&ast.objects)?;
    let _ = check_duplicate_requirements(&ast.requirements)?;
    let _ = check_duplicate_predicates(&ast.predicates)?;
    let _ = check_duplicate_actions(&ast.actions)?;
    let _ = check_duplicate_compound_tasks(&ast.compound_tasks)?;
    let _ = check_duplicate_methods(&ast.methods)?;
    Ok(())
}
