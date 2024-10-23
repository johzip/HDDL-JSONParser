use std::collections::{HashMap, HashSet};

use super::*;

pub struct TDG<'a> {
    tasks: Vec<(&'a str, TaskType)>,
    methods: Vec<HTN<'a>>,
    edges_from_tasks: HashMap<usize, HashSet<usize>>,
    edges_to_tasks: HashMap<usize, HashSet<usize>>,
}

impl<'a> TDG<'a> {
    pub fn new(domain: &'a DomainAST<'a>) -> TDG<'a> {
        // collect task names
        let mut tasks: Vec<(&str, TaskType)> = vec![];
        tasks.extend(
            domain
                .compound_tasks
                .iter()
                .map(|x| (x.name, TaskType::Compound)),
        );
        tasks.extend(domain.actions.iter().map(|x| (x.name, TaskType::Primitive)));

        // edges
        let mut to_methods = HashMap::new();
        let mut to_tasks = HashMap::new();

        // compute index of tasks and methods for efficiency
        let mut task_indices = HashMap::new();
        for (index, (task, _)) in tasks.iter().enumerate() {
            task_indices.insert(*task, index);
            to_methods.insert(index, HashSet::new());
        }

        let mut methods = vec![];
        // collect "task to method" edges
        for (method_index, method) in domain.methods.iter().enumerate() {
            methods.push(method.tn.clone());
            match task_indices.get(method.task_name) {
                Some(task_index) => match to_methods.get_mut(task_index) {
                    Some(set) => {
                        set.insert(method_index);
                    }
                    None => panic!("{} not found", task_index),
                },
                None => panic!("{} is not defined", method.task_name),
            }
        }

        // collect "method to task" edges
        for (method_index, method) in methods.iter().enumerate() {
            let tasks: HashSet<usize> = method
                .subtasks
                .iter()
                .map(|x| match task_indices.get(x.task_symbol) {
                    Some(id) => *id,
                    None => panic!("{} not found", x.task_symbol),
                })
                .collect();
            to_tasks.insert(method_index, tasks);
        }
        TDG {
            tasks: tasks,
            methods: methods,
            edges_from_tasks: to_methods,
            edges_to_tasks: to_tasks,
        }
    }

    pub fn reachable(&self, task_name: &str) -> HashSet<&str> {
        let mut reach_t = HashSet::new();
        let task_index = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(index, (name, t_type))| *name == task_name)
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
            .map(|(_, (name, t_type))| *name)
            .collect()
    }

    pub fn get_recursion_type(&self) -> RecursionType {
        let nullables: HashSet<usize> = self
            .compute_nullables()
            .iter()
            .map(|x| self.get_task_index(&x))
            .collect();
        let mut recursion_type = RecursionType::NonRecursive;
        // DFS over TDG
        let mut stack = vec![];
        // initiating the stack
        // TODO: restrict to those reachable from inital task network
        for (t, methods) in self.edges_from_tasks.iter() {
            for method in methods {
                stack.push(vec![(*t, *method)]);
            }
        }
        // induction
        while let Some(path) = stack.pop() {
            let (_, current_method) = path.iter().last().unwrap();
            for new_task in self.edges_to_tasks.get(current_method).unwrap() {
                let is_cycle = path.iter().map(|(t, _)| t).any(|t| t == new_task);
                if is_cycle {
                    let mut epsilon_prefix = true;
                    // direct recursion
                    if path.len() == 1 {
                        let prefix = self.get_prefix(path[0].0, path[0].1);
                        if prefix.len() != 0 {
                            if prefix.iter().any(|t| !nullables.contains(t)) {
                                epsilon_prefix = false;
                            }
                        }
                    } else {
                        // indirect recursion
                        for i in 1..path.len() - 1 {
                            let (task, _) = &path[i];
                            let (_, method) = &path[i - 1];
                            let prefix = self.get_prefix(*task, *method);
                            if prefix.len() != 0 {
                                if prefix.iter().any(|t| !nullables.contains(t)) {
                                    epsilon_prefix = false;
                                }
                            }
                        }
                    }
                    let mut suffix: Vec<usize> = vec![];
                    // direct recursion
                    if path.len() == 1 {
                        suffix = self.get_suffix(path[0].0, path[0].1);
                    } else {
                        // indirect recursion
                        for i in 1..path.len() - 1 {
                            let (task, _) = &path[i];
                            let (_, method) = &path[i - 1];
                            suffix.extend(self.get_suffix(*task, *method));
                        }
                    }
                    if epsilon_prefix == true {
                        if suffix.len() == 0 {
                            match recursion_type {
                                RecursionType::GrowAndShrinkRecursion => {}
                                _ => {
                                    recursion_type = RecursionType::EmptyRecursion;
                                }
                            }
                        } else {
                            let nullable_suffix = suffix.iter().all(|sym| nullables.contains(sym));
                            match recursion_type {
                                RecursionType::GrowAndShrinkRecursion | RecursionType::EmptyRecursion => {},
                                _ => {
                                    if nullable_suffix {
                                        recursion_type = RecursionType::GrowAndShrinkRecursion;
                                    } else {
                                        recursion_type = RecursionType::GrowingEmptyPrefixRecursion;
                                    }
                                }
                            }
                        }
                    } else {
                        match recursion_type {
                            RecursionType::NonRecursive => {
                                recursion_type = RecursionType::Recursive;
                            }
                            _ => {}
                        }
                    }
                    
                } else if let Some(methods) = self.edges_from_tasks.get(new_task) {
                    for method in methods {
                        let mut new_path = path.clone();
                        new_path.push((*new_task, *method));
                        stack.push(new_path);
                    }
                }
            }
        }
        return recursion_type;
    }

    fn get_prefix(&self, task_index: usize, method_index: usize) -> Vec<usize> {
        let method = &self.methods[method_index];
        let (task, _) = &self.tasks[task_index];
        let mut prefix = vec![];
        match &method.orderings {
            TaskOrdering::Total => {
                for (index, subtask) in method.subtasks.iter().enumerate() {
                    if subtask.task_symbol == *task {
                        return method
                            .subtasks
                            .iter()
                            .take(index)
                            .map(|x| self.get_task_index(&x.task_symbol))
                            .collect();
                    }
                }
                panic!("{} does not exist in {:?}", task, method.subtasks)
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
                let mut suffix: HashSet<&str> = HashSet::new();
                let mut stack = vec![task];
                while let Some(t) = stack.pop() {
                    match adjacency.get(t) {
                        Some(outgoing) => {
                            stack.extend(outgoing.iter());
                            suffix.extend(outgoing.iter());
                        }
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
        let (task, _) = &self.tasks[task_index];
        match &method.orderings {
            TaskOrdering::Total => {
                for (index, subtask) in method.subtasks.iter().enumerate() {
                    if subtask.task_symbol == *task {
                        return method
                            .subtasks
                            .iter()
                            .skip(index + 1)
                            .map(|x| self.get_task_index(&x.task_symbol))
                            .collect();
                    }
                }
                panic!("{} does not exist in {:?}", task, method)
            }
            TaskOrdering::Partial(orderings) => {
                // TODO: test
                todo!()
                // let mut adjacency: HashMap<&str, HashSet<&str>> = HashMap::new();
                // for (e1, e2) in orderings {
                //     if adjacency.contains_key(e1) {
                //         let neighbors: &mut HashSet<&str> = adjacency.get_mut(e1).unwrap();
                //         neighbors.insert(e2);
                //     } else {
                //         adjacency.insert(e1, HashSet::from([*e2]));
                //     }
                // }
                // let mut suffix: HashSet<&str> = HashSet::new();
                // let mut stack = vec![task];
                // while let Some(t) = stack.pop() {
                //     match adjacency.get(t) {
                //         Some(outgoing) => {
                //             stack.extend(outgoing.iter());
                //             suffix.extend(outgoing.iter());
                //         }
                //         None => {}
                //     }
                // }
                // return suffix;
            }
        }
    }

    fn get_task_index(&self, task_name: &str) -> usize {
        self.tasks
            .iter()
            .enumerate()
            .find(|(_, (name, t_type))| *name == task_name)
            .unwrap()
            .0
    }

    pub fn compute_nullables(&self) -> HashSet<&'a str> {
        // nullable base case
        let mut nullables: HashSet<usize> = self
            .edges_from_tasks
            .iter()
            .filter_map(|(task, methods)| {
                for method in methods.iter() {
                    let tasks = self.edges_to_tasks.get(method).unwrap();
                    if tasks.len() == 0 {
                        return Some(*task);
                    }
                }
                None
            })
            .collect();

        // unit reachability base case
        let mut unit_reachability: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (t, t_type) in self.tasks.iter() {
            match *t_type {
                TaskType::Primitive => {}
                TaskType::Compound => {
                    let task_index = self.get_task_index(t);
                    let mut value = HashSet::from([task_index]);
                    if let Some(methods) = self.edges_from_tasks.get(&task_index) {
                        for method in methods {
                            let tasks = self.edges_to_tasks.get(method).unwrap();
                            if tasks.len() == 1 {
                                value.insert(*tasks.iter().next().unwrap());
                            }
                        }
                    }

                    unit_reachability.insert(task_index, value);
                }
            }
        }
        let mut changed_nullables = true;
        let mut changed_unit_reachability = true;
        let mut new_nullables = HashSet::new();
        let mut new_unit_reachable: HashMap<usize, HashSet<usize>> = HashMap::new();
        while changed_nullables || changed_unit_reachability {
            // nullables induction step
            for (t, methods) in self.edges_from_tasks.iter() {
                for method in methods {
                    if let Some(tasks) = self.edges_to_tasks.get(method) {
                        if tasks.iter().all(|x| match unit_reachability.get(x) {
                            Some(set) => {
                                let intersection: HashSet<&usize> =
                                    set.intersection(&nullables).collect();
                                intersection.len() != 0
                            }
                            None => false,
                        }) {
                            new_nullables.insert(*t);
                        }
                    }
                }
            }

            // unit reachability induction step
            for (c, previous_reachables) in unit_reachability.iter() {
                let mut change = previous_reachables.clone();
                for previous_reachable in previous_reachables {
                    match unit_reachability.get(previous_reachable) {
                        Some(tasks) => {
                            change = change.union(tasks).cloned().collect();
                        }
                        None => {}
                    }
                }
                for method in self.edges_from_tasks.get(c).unwrap() {
                    if let Some(tasks) = self.edges_to_tasks.get(method) {
                        let mut not_nullable = None;
                        for task in tasks {
                            if !nullables.contains(task) {
                                if not_nullable.is_none() {
                                    not_nullable = Some(*task)
                                } else {
                                    break;
                                }
                            }
                        }
                        if let Some(val) = not_nullable {
                            change.insert(val);
                        }
                    }
                }
                if change == *previous_reachables {
                    changed_unit_reachability = false;
                } else {
                    new_unit_reachable.insert(*c, change);
                }
            }

            // commit to changes
            //// nullables
            if new_nullables.len() == nullables.len() {
                changed_nullables = false;
            } else {
                for n in new_nullables.iter() {
                    nullables.insert(*n);
                }
            }
            //// unit reachability
            for (task, new_reachable) in new_unit_reachable.iter() {
                let prev = unit_reachability.get_mut(&task).unwrap();
                prev.extend(new_reachable);
            }
        }
        let mut result = HashSet::new();
        for task_index in nullables {
            result.insert(self.tasks[task_index].0);
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum TaskType {
    Primitive,
    Compound,
}
