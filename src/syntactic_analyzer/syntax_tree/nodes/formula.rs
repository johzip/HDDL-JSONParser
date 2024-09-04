use super::*;

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
    Exists(Vec<&'a str>, Vec<Box<Formula<'a>>>),
    // ∀vars: formula 
    ForAll(Vec<Variable<'a>>, Box<Formula<'a>>),
    // formula = formula'
    Equals(&'a str, &'a str)
}

impl <'a> Formula<'a> {
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
            Formula::And(new_formula) |
            Formula::Or(new_formula) |
            Formula::Xor(new_formula) => {
                for f in new_formula {
                    predicates.extend(f.get_predicates().iter());
                }
            }
            Formula::ForAll(_, new_formula) => {
                predicates.extend(new_formula.get_predicates().iter());
            }
            Formula::Equals(_, _) => { }
            // TODO: add support for imply, and exists
            _ => {
                panic!()
            }
        }
        return predicates;
    }
}