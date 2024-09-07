use std::collections::HashMap;

use super::*;

#[derive(Clone)]
pub enum Formula<'a> {
    Empty,
    Atom(Predicate<'a>),
    Not(Box<Formula<'a>>),
    And(Vec<Box<Formula<'a>>>),
    Or(Vec<Box<Formula<'a>>>),
    Xor(Vec<Box<Formula<'a>>>),
    // formula -> formula'
    Imply(Vec<Box<Formula<'a>>>, Vec<Box<Formula<'a>>>),
    // ∃vars: formula
    Exists(Vec<Variable<'a>>, Box<Formula<'a>>),
    // ∀vars: formula
    ForAll(Vec<Variable<'a>>, Box<Formula<'a>>),
    // formula = formula'
    Equals(&'a str, &'a str),
}

impl<'a> Formula<'a> {
    pub fn get_predicates(&self) -> Vec<&Predicate<'a>> {
        let mut predicates = vec![];
        match &*self {
            Formula::Empty => {}
            Formula::Atom(predicate) => {
                predicates.push(predicate);
            }
            Formula::Not(new_formula) => {
                predicates.extend(new_formula.get_predicates().iter());
            }
            Formula::And(new_formula) | Formula::Or(new_formula) | Formula::Xor(new_formula) => {
                for f in new_formula {
                    predicates.extend(f.get_predicates().iter());
                }
            }
            Formula::ForAll(_, new_formula) => {
                predicates.extend(new_formula.get_predicates().iter());
            }
            Formula::Equals(_, _) => {}
            // TODO: add support for imply, and exists
            _ => {
                panic!()
            }
        }
        return predicates;
    }

    // TODO: test
    pub fn to_cnf(&self) -> Formula<'a> {
        return self.simplify().to_nnf().distribute_disjunction();
    }

    fn simplify(&self) -> Formula<'a> {
        match self {
            Formula::Empty => Formula::Empty,
            Formula::Atom(_) => self.clone(),
            Formula::Not(f) => Formula::Not(Box::new(f.simplify())),
            Formula::And(fs) => Formula::And(fs.iter().map(|f| Box::new(f.simplify())).collect()),
            Formula::Or(fs) => Formula::Or(fs.iter().map(|f| Box::new(f.simplify())).collect()),
            Formula::Xor(fs) => {
                // Convert XOR to a combination of AND, OR, and NOT
                let mut result = Vec::new();
                for (i, f) in fs.iter().enumerate() {
                    let mut clause = Vec::new();
                    for (j, g) in fs.iter().enumerate() {
                        if i == j {
                            clause.push(Box::new(g.simplify()));
                        } else {
                            clause.push(Box::new(Formula::Not(Box::new(g.simplify()))));
                        }
                    }
                    result.push(Box::new(Formula::Or(clause)));
                }
                Formula::And(result)
            }
            Formula::Imply(antecedents, consequents) => {
                let not_antecedents: Box<Formula<'a>> = Box::new(Formula::And(
                    antecedents.iter().map(|f| Box::new(f.simplify())).collect(),
                ));
                let consequents: Box<Formula<'a>> = Box::new(Formula::And(
                    consequents.iter().map(|f| Box::new(f.simplify())).collect(),
                ));
                Formula::Or(vec![not_antecedents, consequents])
            }
            Formula::Exists(quantifier, f) => {
                Formula::Exists(quantifier.clone(), Box::new(f.simplify()))
            }
            Formula::ForAll(quantifier, f) => {
                Formula::ForAll(quantifier.clone(), Box::new(f.simplify()))
            }
            // TODO: Add support
            Formula::Equals(_, _) => panic!(),
        }
    }

    fn to_nnf(&self) -> Formula<'a> {
        match self {
            Formula::Empty => Formula::Empty,
            Formula::Atom(_) => self.clone(),
            Formula::Not(f) => match &**f {
                Formula::Empty => self.clone(),
                Formula::Atom(p) => self.clone(),
                Formula::Not(g) => g.to_nnf(),
                Formula::And(fs) => Formula::Or(
                    fs.iter()
                        .map(|f| Box::new(Formula::Not(Box::new(f.to_nnf()))))
                        .collect(),
                ),
                Formula::Or(fs) => Formula::And(
                    fs.iter()
                        .map(|f| Box::new(Formula::Not(Box::new(f.to_nnf()))))
                        .collect(),
                ),
                // TODO: test
                Formula::Exists(quantifier, f) => Formula::ForAll(
                    quantifier.clone(),
                    Box::new(Formula::Not(Box::new(f.to_nnf()))),
                ),
                // TODO: test
                Formula::ForAll(quantifier, f) => Formula::Exists(
                    quantifier.clone(),
                    Box::new(Formula::Not(Box::new(f.to_nnf()))),
                ),
                //
                Formula::Xor(_) | Formula::Imply(_, _) => unreachable!(),
                // TODO: add support
                Formula::Equals(_, _) => panic!(),
            },
            Formula::And(fs) => Formula::And(fs.iter().map(|f| Box::new(f.to_nnf())).collect()),
            Formula::Or(fs) => Formula::Or(fs.iter().map(|f| Box::new(f.to_nnf())).collect()),
            Formula::ForAll(quantifier, f) => {
                Formula::ForAll(quantifier.clone(), Box::new(f.to_nnf()))
            }
            Formula::Exists(quantifier, f) => {
                Formula::Exists(quantifier.clone(), Box::new(f.to_nnf()))
            }
            _ => unreachable!("Formula is not simplified"),
        }
    }

    fn distribute_disjunction(&self) -> Formula<'a> {
        match self {
            Formula::Empty | Formula::Atom(_) | Formula::Not(_) => self.clone(),
            Formula::And(fs) => Formula::And(
                fs.iter()
                    .map(|f| Box::new(f.distribute_disjunction()))
                    .collect(),
            ),
            Formula::Or(fs) => {
                let distributed: Vec<Box<Formula<'a>>> = fs
                    .iter()
                    .map(|f| Box::new(f.distribute_disjunction()))
                    .collect();
                let mut result = Vec::new();
                let mut queue = vec![distributed];
                while let Some(current) = queue.pop() {
                    if let Some(position) =
                        current.iter().position(|f| matches!(**f, Formula::And(_)))
                    {
                        let Formula::And(conjuncts) = &*current[position] else {
                            unreachable!()
                        };
                        for conjunct in conjuncts {
                            let mut new_formula = current.clone();
                            new_formula[position] = conjunct.clone();
                            queue.push(new_formula);
                        }
                    } else {
                        result.push(Formula::Or(current));
                    }
                }
                if result.len() == 1 {
                    result.pop().unwrap()
                } else {
                    Formula::And(result.into_iter().map(Box::new).collect())
                }
            }
            _ => unreachable!("formula is not simplified"),
        }
    }

    pub fn to_clauses(&self) -> (u32, Vec<Vec<i32>>) {
        let mut literal_ids = HashMap::new();
        let mut clauses = vec![];
        let mut count = 1;
        match self.to_cnf() {
            Formula::And(subformula) => {
                for f in subformula {
                    let mut clause = vec![];
                    match *f {
                        Formula::Empty => {}
                        Formula::Not(pred_box) => {
                            if let Formula::Atom(predicate) = *pred_box {
                                if !literal_ids.contains_key(predicate.name) {
                                    literal_ids.insert(predicate.name, count);
                                    count+=1;
                                }
                                clause.push(-1 * literal_ids.get(predicate.name).unwrap());
                            } else {
                                panic!("not simplified")
                            }
                        }
                        Formula::Or(disjuncts) => {
                            for disjunct in disjuncts {
                                match *disjunct {
                                    Formula::Atom(predicate) => {
                                        if !literal_ids.contains_key(predicate.name) {
                                            literal_ids.insert(predicate.name, count);
                                            count+=1;
                                        }
                                        clause.push(*literal_ids.get(predicate.name).unwrap());
                                    },
                                    Formula::Not(pred_box) => {
                                        if let Formula::Atom(predicate) = *pred_box {
                                            if !literal_ids.contains_key(predicate.name) {
                                                literal_ids.insert(predicate.name, count);
                                                count+=1;
                                            }
                                            clause.push(-1 * literal_ids.get(predicate.name).unwrap());
                                        } else {
                                            panic!("not simplified")
                                        }
                                    }
                                    _ => panic!("not in CNF")
                                }
                            }
                        }
                        _ => panic!("not in CNF")

                    }
                    clauses.push(clause);
                }
            }
            Formula::Or(disjuncts) => {
                for disjunct in disjuncts.iter() {
                    let mut clause = vec![];
                    match &**disjunct {
                        Formula::Atom(predicate) => {
                            if !literal_ids.contains_key(predicate.name) {
                                literal_ids.insert(predicate.name, count);
                                count+=1;
                            }
                        },
                        Formula::Not(pred_box) => {
                            if let Formula::Atom(predicate) = &**pred_box {
                                if !literal_ids.contains_key(predicate.name) {
                                    literal_ids.insert(predicate.name, count);
                                    count+=1;
                                }
                                clause.push(-1 * literal_ids.get(predicate.name).unwrap());
                            } else {
                                panic!("not simplified")
                            }
                        }
                        _ => panic!("not in CNF")
                    }
                    return (disjuncts.len() as u32, vec![clause]);
                }
            }
            Formula::Not(p) => {
                if let Formula::Atom(predicate) = *p {
                    if !literal_ids.contains_key(predicate.name) {
                        literal_ids.insert(predicate.name, count);
                        count+=1;
                    }

                }
            }
            _ => panic!()
        }
        ((count - 1) as u32, clauses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn demorgan_rules_test() {
        // ~(a^b) = ~a v ~b
        let f1 = Formula::Not(Box::new(Formula::And(vec![
            Box::new(Formula::Atom(Predicate {
                name: "a",
                variables: vec![],
            })),
            Box::new(Formula::Atom(Predicate {
                name: "b",
                variables: vec![],
            })),
        ])));
        let cnf = f1.to_cnf();
        match cnf {
            Formula::Or(a) => {
                match &*a[0] {
                    Formula::Not(p) => match &**p {
                        Formula::Atom(p) => {
                            assert_eq!(p.name, "a")
                        }
                        _ => panic!("wrong result"),
                    },
                    _ => panic!("wrong result"),
                }
                match &*a[1] {
                    Formula::Not(p) => match &**p {
                        Formula::Atom(p) => {
                            assert_eq!(p.name, "b")
                        }
                        _ => panic!("wrong result"),
                    },
                    _ => panic!("wrong result"),
                }
            }
            _ => panic!("wrong result"),
        }

        // ~(a v b) = ~a ^ ~b
        let f2 = Formula::Not(Box::new(Formula::Or(vec![
            Box::new(Formula::Atom(Predicate {
                name: "a",
                variables: vec![],
            })),
            Box::new(Formula::Atom(Predicate {
                name: "b",
                variables: vec![],
            })),
        ])));
        let cnf = f2.to_cnf();
        match cnf {
            Formula::And(a) => {
                match &*a[0] {
                    Formula::Not(p) => match &**p {
                        Formula::Atom(p) => {
                            assert_eq!(p.name, "a")
                        }
                        _ => panic!("wrong result"),
                    },
                    _ => panic!("wrong result"),
                }
                match &*a[1] {
                    Formula::Not(p) => match &**p {
                        Formula::Atom(p) => {
                            assert_eq!(p.name, "b")
                        }
                        _ => panic!("wrong result"),
                    },
                    _ => panic!("wrong result"),
                }
            }
            _ => panic!("wrong result"),
        }
    }

    #[test]
    pub fn cnf_clause_test() {
        let cnf = Formula::And(vec![
            Box::new(Formula::Or(vec![
                Box::new(Formula::Atom(Predicate { name: "a", variables: vec![] })),
                Box::new(Formula::Not(Box::new(Formula::Atom(Predicate { name: "c", variables: vec![] })))),
            ])),
            Box::new(Formula::Or(vec![
                Box::new(Formula::Atom(Predicate { name: "b", variables: vec![] })),
                Box::new(Formula::Atom(Predicate { name: "c", variables: vec![] })),
                Box::new(Formula::Not(Box::new(Formula::Atom(Predicate { name: "a", variables: vec![] })))),
            ])),
        ]);
        let (var_count, clauses) = cnf.to_clauses();
        assert_eq!(var_count, 3);
        assert_eq!(clauses.len(), 2);
        assert_eq!(clauses[0], vec![1, -2]);
        assert_eq!(clauses[1], vec![3, 2, -1]);
    }
}
