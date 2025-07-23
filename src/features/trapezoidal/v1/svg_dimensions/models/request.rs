use serde::Deserialize;

use crate::shared::enums::{theme::Theme, Language, ThreadType, Unit};

#[derive(Debug, Deserialize)]
pub struct RequestSvgDimension {
    #[serde(rename = "type")]
    pub thread_type: ThreadType,
    pub theme: Theme,
    pub tolerance: String,
    pub diameter: i32,
    pub pitch: f64,
    pub language: Language,
    pub units: Unit,
    pub precision: Option<usize>,
}
