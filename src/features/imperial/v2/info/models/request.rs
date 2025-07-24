use serde::Deserialize;

use crate::shared::enums::{Language, ThreadType, Unit};

/// Структура для извлечения параметров запроса
#[derive(Deserialize, Debug, Clone)]
pub struct RequestV2ImperialInfo {
    pub diameter: String,
    pub tpi: f64,
    pub series: String, // Will match with either class_m or class_f based on type
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub language: Language, // Ожидает "ru" или "en"
    pub units: Unit,
    pub precision: Option<usize>,
}
