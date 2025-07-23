use serde::{Deserialize, Serialize};

use crate::shared::enums::{Language, Unit};

// Request models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestPipeInfo {
    pub id: i32,
    pub language: Language,
    pub units: Unit,
    pub precision: usize,
}
