use std::collections::{HashMap, HashSet};

pub struct TypedList <'a> {
    pub variables: Vec<&'a str>,
    // mapping from arg name to its type (if typing exists)
    pub variable_types: Option<HashMap<&'a str, &'a str>> 
}