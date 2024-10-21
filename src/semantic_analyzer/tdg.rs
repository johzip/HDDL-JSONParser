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

impl<'a> TDG<'a> {
    pub fn new(domain: &'a DomainAST<'a>, init_tn: HTN<'a>) -> TDG<'a> {
        // collect task names
        let mut tasks: Vec<&str> = vec!["tdg_init"];
        tasks.extend(domain.compound_tasks.iter().map(|x| x.name));
        tasks.extend(domain.actions.iter().map(|x| x.name));

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
        for (method_index, method) in domain.methods.iter().enumerate() {
            // TODO: remove the clone part
            methods.push(method.tn.clone());
            match task_indices.get(method.task_name) {
                Some(task_index) => match from_task_edges.get_mut(task_index) {
                    Some(set) => {
                        set.insert(method_index + 1);
                    }
                    None => panic!("{} not found", task_index),
                },
                None => panic!("{} is not defined", method.task_name),
            }
        }

        // collect "to task" edges
        for (method_index, method) in methods.iter().enumerate() {
            let tasks: HashSet<usize> = method
                .subtasks
                .iter()
                .map(|x| match task_indices.get(x.task_symbol) {
                    Some(id) => *id,
                    None => panic!(),
                })
                .collect();
            to_task_edges.insert(method_index, tasks);
        }
        TDG {
            tasks: tasks,
            methods: methods,
            edges_from_tasks: from_task_edges,
            edges_to_tasks: to_task_edges,
        }
    }

    pub fn reachable(&self, task_name: &str) -> HashSet<&str> {
        let mut reach_t = HashSet::new();
        let task_index = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(index, name)| **name == task_name)
            .next()
            .unwrap()
            .0;
        reach_t.insert(task_index);
        match self.edges_from_tasks.get(&task_index) {
            Some(methods) => {
                for m in methods {
                    reach_t.extend(self.edges_to_tasks.get(m).unwrap().iter());
                }
            }
            None => {
                panic!()
            }
        }
        self.tasks
            .iter()
            .enumerate()
            .filter(|(index, _)| reach_t.contains(index))
            .map(|(_, name)| *name)
            .collect()
    }

    pub fn get_recursion_type(&self) -> RecursionType {
        // TODO: fix nullables
        let nullables: HashSet<usize> = HashSet::new();
        let mut recursion_type = RecursionType::NonRecursive;
        // DFS over TDG
        let mut stack = vec![vec![(0, 0)]];
        while let Some(path) = stack.pop() {
            let (_, method_index) = path.iter().last().unwrap();
            for reachable_task_index in self.edges_to_tasks.get(method_index).unwrap() {
                for reachable_method_index in
                    self.edges_from_tasks.get(reachable_task_index).unwrap()
                {
                    let new_item = (*reachable_task_index, *reachable_method_index);
                    let mut new_path = path.clone();
                    new_path.push(new_item);
                    if !path.contains(&new_item) {
                        stack.push(new_path);
                    } else {
                        let epsilon_prefix = {
                            new_path.iter().all(|(t, m)| {
                                let prefix = self.get_prefix(*t, *m);
                                if prefix.len() == 0 {
                                    return true;
                                } else {
                                    for symbol in prefix {
                                        if !nullables.contains(&symbol) {
                                            return false;
                                        }
                                    }
                                    return true;
                                }
                            })
                        };
                        if epsilon_prefix == false {
                            recursion_type = RecursionType::Recursive;
                        } else {
                            recursion_type = RecursionType::EmptyPrefixRecursion;
                        }
                        let mut suffix: Vec<usize> = vec![];
                        for (t, m) in new_path {
                            suffix.extend(self.get_suffix(t, m).iter());
                        }
                        if suffix.len() == 0 {
                            recursion_type = RecursionType::EmptyRecursion;
                        } else {
                            if suffix.iter().all(|sym| nullables.contains(sym)) {
                                recursion_type = RecursionType::GrowingEmptyPrefixRecursion;
                            } else {
                                recursion_type = RecursionType::GrowAndShrinkRecursion;
                            }
                        }
                    }
                }
            }
        }
        return recursion_type;
    }

    fn get_prefix(&self, task_index: usize, method_index: usize) -> Vec<usize> {
        let method = &self.methods[method_index];
        let task = self.tasks[task_index];
        let mut prefix = vec![];
        match &method.orderings {
            TaskOrdering::Total => {
                let first_occurance = method
                    .subtasks
                    .iter()
                    .enumerate()
                    .filter(|(_, name)| name.task_symbol == task)
                    .map(|(index, _)| index)
                    .next()
                    .unwrap();
                for (index, subtask) in method.subtasks.iter().enumerate() {
                    if index == first_occurance {
                        break;
                    }
                    prefix.push(self.get_task_index(&subtask.task_symbol));
                }
            }
            TaskOrdering::Partial(orderings) => {
                // TODO: test
                let mut adjacency: HashMap<&str, HashSet<&str>> = HashMap::new();
                for (e1, e2) in orderings {
                    if adjacency.contains_key(e1) {
                        let neighbors: &mut HashSet<&str> = adjacency.get_mut(e1).unwrap();
                        neighbors.insert(e2);
                    } else {
                        adjacency.insert(e1, HashSet::from([*e2]));
                    }
                }
                let mut suffix: HashSet<&str>= HashSet::new();
                let mut stack = vec![task];
                while let Some(t) = stack.pop() {
                    match adjacency.get(&t) {
                        Some(outgoing) => {
                            stack.extend(outgoing.iter());
                            suffix.extend(outgoing.iter());
                        },
                        None => {}
                    }
                }
                for subtask in &method.subtasks {
                    if !suffix.contains(subtask.task_symbol) {
                        prefix.push(self.get_task_index(subtask.task_symbol));
                    }
                }
            }
        }
        return prefix;
    }

    fn get_suffix(&self, task_index: usize, method_index: usize) -> Vec<usize> {
        let method = &self.methods[method_index];
        let task = self.tasks[task_index];
        let mut suffix = vec![];
        match &method.orderings {
            TaskOrdering::Total => {
                let first_occurance = method
                    .subtasks
                    .iter()
                    .enumerate()
                    .filter(|(_, name)| name.task_symbol == task)
                    .map(|(index, _)| index)
                    .next()
                    .unwrap();
                for (index, subtask) in method.subtasks.iter().enumerate() {
                    if (index <= first_occurance) {
                        continue;
                    }
                    suffix.push(self.get_task_index(&subtask.task_symbol));
                }
            }
            TaskOrdering::Partial(orderings) => {
                // TODO: test
                let mut adjacency: HashMap<&str, HashSet<&str>> = HashMap::new();
                for (e1, e2) in orderings {
                    if adjacency.contains_key(e1) {
                        let neighbors: &mut HashSet<&str> = adjacency.get_mut(e1).unwrap();
                        neighbors.insert(e2);
                    } else {
                        adjacency.insert(e1, HashSet::from([*e2]));
                    }
                }
                let mut suffix: HashSet<&str>= HashSet::new();
                let mut stack = vec![task];
                while let Some(t) = stack.pop() {
                    match adjacency.get(&t) {
                        Some(outgoing) => {
                            stack.extend(outgoing.iter());
                            suffix.extend(outgoing.iter());
                        },
                        None => {}
                    }
                }
            }
        }
        return suffix;
    }

    fn get_task_index(&self, task_name: &str) -> usize {
        self.tasks
            .iter()
            .enumerate()
            .find(|(index, name)| **name == task_name)
            .unwrap()
            .0
    }
}
