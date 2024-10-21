use std::collections::{HashMap, HashSet};

use super::*;

pub struct TDG<'a> {
    // index 0 is reserved for the initial task network
    tasks: Vec<&'a str>,
    // index 0 is reserved for the initial task decomposition
    methods: Vec<HTN<'a>>,
    edges_from_tasks: HashMap<usize, HashSet<usize>>,
    edges_to_tasks: HashMap<usize, HashSet<usize>>,
}

impl <'a> TDG <'a> {
    pub fn new(domain: &'a DomainAST<'a>, init_tn: HTN<'a>) -> TDG<'a> {
        // collect task names
        let mut tasks: Vec<&str> = vec!["tdg_init"];
        tasks.extend(domain.compound_tasks.iter().map(|x| {
            x.name
        }));
        tasks.extend(domain.actions.iter().map(|x| {
            x.name
        }));

        // edges
        let mut from_task_edges = HashMap::new();
        let mut to_task_edges = HashMap::new();

        // compute index of tasks and methods for efficiency
        let mut task_indices = HashMap::new();
        for (index, task) in tasks.iter().enumerate() {
            task_indices.insert(*task, index);
            from_task_edges.insert(index, HashSet::new());
        }
        
        let mut methods = vec![init_tn.clone()];
        // collect "from task" edges
        for (method_index,method) in domain.methods.iter().enumerate() {
            // TODO: remove the clone part
            methods.push(method.tn.clone());
            match task_indices.get(method.task_name) {
                Some(task_index) => {
                    match from_task_edges.get_mut(task_index) {
                        Some(set) => {
                            set.insert(method_index + 1);
                        }
                        None => panic!("{} not found", task_index)
                    }
                }
                None => panic!("{} is not defined", method.task_name)
            }
        }

        // collect "to task" edges
        for (method_index, method) in methods.iter().enumerate() {
            let tasks: HashSet<usize> = method.subtasks.iter().map(|x| {
                match task_indices.get(x.task_symbol) {
                    Some(id) => *id,
                    None => panic!()
                }
            }).collect();
            to_task_edges.insert(method_index, tasks);
        }
        TDG {
            tasks: tasks,
            methods: methods,
            edges_from_tasks: from_task_edges,
            edges_to_tasks: to_task_edges
        }
    }

    pub fn reachable(&self, task_name: &str) -> HashSet<&str> {
        let mut reach_t = HashSet::new();
        let task_index = self.tasks.iter().enumerate()
            .filter(|(index, name)| **name == task_name)
            .next()
            .unwrap().0;
        reach_t.insert(task_index);
        match self.edges_from_tasks.get(&task_index) {
            Some(methods) => {
                for m in methods {
                    reach_t.extend(self.edges_to_tasks
                        .get(m)
                        .unwrap()
                        .iter()
                    );
                }
            }
            None => { panic!() }
        }
        self.tasks.iter().enumerate().filter(|(index, _)| {
            reach_t.contains(index)
        }).map(|(_, name)| {
            *name
        }).collect()
    }
}