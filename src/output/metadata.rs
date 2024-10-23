use std::fmt::{Display, Formatter, Error};

#[derive(PartialEq, Eq, Debug)]
pub enum RecursionType {
    NonRecursive,
    Recursive,
    EmptyRecursion,
    GrowingEmptyPrefixRecursion,
    GrowAndShrinkRecursion
}

impl Display for RecursionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            RecursionType::NonRecursive => write!(f, "Non-Recursive"),
            RecursionType::Recursive => write!(f, "Recursive"),
            RecursionType::EmptyRecursion => write!(f, "Empty Cycle"),
            RecursionType::GrowingEmptyPrefixRecursion => write!(f, "Growing Empty Prefix Cycle"),
            RecursionType::GrowAndShrinkRecursion => write!(f, "Grow and Shrink Cycle")
        }
    }
}


pub struct MetaData {
    pub recursion: RecursionType,
    pub nullables: Vec<String>,
    pub domain_name: String,
}

impl Display for MetaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{} Description:", self.domain_name)?;
        writeln!(f, "\tHierarchy type: {}", self.recursion)?;
        if self.nullables.len() == 0 {
            writeln!(f, "\tNullable Tasks: None")?;
        } else {
            writeln!(f, "\tNullable Tasks:")?;
            for nullable in self.nullables.iter() {
                writeln!(f, "\t\t{}", nullable)?
            }
        }
        Ok(())
    }
}