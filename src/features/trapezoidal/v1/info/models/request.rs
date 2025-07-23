use serde::{Deserialize, Serialize};

use crate::shared::enums::{Language, ThreadType, Unit};

// Request models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestTrapezoidalInfo {
    pub diameter: i32,
    pub pitch: f64,
    pub tolerance: String,
    #[serde(rename = "type")]
    pub type_thread: ThreadType,
    pub language: Language,
    pub units: Unit,
    pub precision: Option<usize>,
}
