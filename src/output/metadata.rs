pub enum RecursionType {
    NonRecursive,
    Recursive,
    EmptyPrefixRecursion,
    EmptyRecursion,
    GrowingEmptyPrefixRecursion,
    GrowAndShrinkRecursion
}

pub struct MetaData {
    recursion: Option<RecursionType>,
    domain_name: String,
}