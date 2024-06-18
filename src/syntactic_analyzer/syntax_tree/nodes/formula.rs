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
    ForAll(Vec<&'a str>, Vec<Box<Formula<'a>>>),
    // formula = formula'
    Equals(Vec<Box<Formula<'a>>>, Vec<Box<Formula<'a>>>)
}