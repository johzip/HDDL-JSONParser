use super::*;
use std::collections::{HashMap, HashSet};

pub struct SyntaxTree<'a> {
    pub objects: Vec<&'a str>,
    // TODO: convert to type hierarchy
    pub types: Option<Vec<&'a str>>,
    pub object_types: Option<HashMap<&'a str, &'a str>>,
    pub requirements: Vec<RequirementType>,
    // mapping from predicate name to its arguments
    pub predicates: Vec<Predicate<'a>>,
    // mapping from compound task name to its parameters
    pub compound_tasks: Vec<Task<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub fn new() -> SyntaxTree<'a> {
        SyntaxTree {
            objects: vec![],
            types: None,
            object_types: None,
            requirements: vec![],
            predicates: vec![],
            compound_tasks: vec![],
        }
    }
    pub fn add_object(&mut self, object: &'a str) {
        self.objects.push(object);
    }
    pub fn add_typed_object(&mut self, object: &'a str, object_type: &'a str) {
        self.objects.push(object);
        match &mut self.types {
            Some(types) => {
                types.push(object_type);
                self.object_types
                    .as_mut()
                    .unwrap()
                    .insert(object, object_type);
            }
            None => {
                self.types = Some(vec![object_type]);
                self.object_types = Some(HashMap::from([(object, object_type)]));
            }
        }
    }

    pub fn add_requirement(&mut self, req: RequirementType) {
        self.requirements.push(req);
    }

    pub fn add_predicate(&mut self, predicate: Predicate<'a>) {
        self.predicates.push(predicate);
    }

    pub fn add_compound_task(&mut self, task: Task<'a>) {
        self.compound_tasks.push(task);
    }
}
