use serde::Deserialize;

use crate::shared::enums::{theme::Theme, Language, ThreadType, Unit};

#[derive(Debug, Deserialize)]
pub struct RequestSvgDimension {
    #[serde(rename = "type")]
    pub thread_type: ThreadType,
    pub theme: Theme,
    pub tolerance: String,
    pub diameter: String, // Imperial uses string for fractions like "1/4", "#10"
    pub tpi: f64,         // Threads per inch instead of pitch
    pub language: Language,
    pub units: Unit,
    pub precision: Option<usize>,
}
