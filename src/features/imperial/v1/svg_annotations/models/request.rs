use serde::Deserialize;

use crate::shared::enums::{theme::Theme, Language, ThreadType};

#[derive(Debug, Deserialize)]
pub struct RequestSvgAnnotation {
    #[serde(rename = "type")]
    pub thread_type: ThreadType,

    pub theme: Theme,

    pub language: Language,
}