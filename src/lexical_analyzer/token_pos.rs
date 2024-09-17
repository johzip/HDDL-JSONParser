#[derive(Debug, Clone, Copy)]
pub struct TokenPosition {
    pub line: u32,
    pub column: (u32, Option<u32>)
}