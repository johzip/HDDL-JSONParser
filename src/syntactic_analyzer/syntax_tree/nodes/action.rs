use super::*;

pub struct Action<'a> {
    pub name: &'a str,
    pub parameters: Arguements<'a>,
    pub preconditions: Formula<'a>,
    pub effects: Formula<'a>
}