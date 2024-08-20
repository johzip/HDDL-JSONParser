use super::*;
use std::collections::HashMap;

pub struct SyntaxTree<'a> {
    pub objects: Vec<Variable<'a>>,
    pub types: Option<Vec<Variable<'a>>>,
    pub constants: Option<Vec<Variable<'a>>>,
    pub requirements: Vec<RequirementType>,
    // mapping from predicate name to its arguments
    pub predicates: Vec<Predicate<'a>>,
    // mapping from compound task name to its parameters
    pub compound_tasks: Vec<Task<'a>>,
    pub init_tn: Option<InitialTaskNetwork<'a>>,
    pub methods: Vec<Method<'a>>,
    pub actions: Vec<Action<'a>>,
    pub init_state: Vec<Predicate<'a>>,
    pub goal: Option<Formula<'a>>
}

impl<'a> SyntaxTree<'a> {
    pub fn new() -> SyntaxTree<'a> {
        SyntaxTree {
            objects: vec![],
            types: None,
            constants: None,
            requirements: vec![],
            predicates: vec![],
            compound_tasks: vec![],
            init_tn: None,
            methods: vec![],
            actions: vec![],
            init_state: vec![],
            goal: None
        }
    }
    pub fn add_object(&mut self, name: &'a str) {
        let object = Variable::new(name, None);
        self.objects.push(object);
    }
    pub fn add_typed_object(&mut self, name: &'a str, object_type: &'a str) {
        let object = Variable::new(name, Some(object_type));
        self.objects.push(object);
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

    pub fn add_init_tn(&mut self, tn: InitialTaskNetwork<'a>) {
        self.init_tn = Some(tn);
    }

    pub fn add_method(&mut self, method: Method<'a>) {
        self.methods.push(method)
    }

    pub fn add_action(&mut self, action: Action<'a>) {
        self.actions.push(action);
    }

    pub fn add_init_state(&mut self, state: Vec<Predicate<'a>>) {
        self.init_state = state;
    }

    pub fn add_goal(&mut self, goal: Formula<'a>) {
        self.goal = Some(goal);
    }

    pub fn add_var_type(&mut self, var: Variable<'a>){
        match self.types.as_mut() {
            Some(t) => {
                t.push(var);
            }
            None => {
                self.types = Some(vec![var])
            }
        }
    }

    pub fn add_constant(&mut self, constant: Variable<'a>){
        match self.constants.as_mut() {
            Some(c) => {
                c.push(constant);
            }
            None => {
                self.constants = Some(vec![constant])
            }
        }
    }
}
