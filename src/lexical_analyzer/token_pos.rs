use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TokenPosition {
    pub line: u32,
}