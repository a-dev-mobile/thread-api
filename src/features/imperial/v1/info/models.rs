use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::shared::enums::{Language, ThreadType, Unit};

/// Структура для извлечения параметров запроса
#[derive(Deserialize, Debug, Clone)]
pub struct RequestParams {
    pub diameter: String,
    pub tpi: f64,
    pub series: String, // Will match with either class_m or class_f based on type
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub language: Language, // Ожидает "ru" или "en"
    pub units: Unit,
    pub precision: Option<usize>,
}

/// Модель данных из базы данных
#[derive(Serialize, FromRow, Debug, Clone)]
pub struct DbModel {
    pub id: i64,
    pub diameter: String,
    pub diameter_2: f64, // Используем как 'd'
    pub tpi: f64,
    pub series_designation: String,
    pub class_m: String,
    pub class_f: String,
    pub allowance: f64,
    pub major_diam_max_m: f64,
    pub major_diam_min_m: f64,
    pub major_diam_min2_m: Option<f64>,
    pub pitch_diameter_max_m: f64,
    pub pitch_diameter_min_m: f64,
    pub pitch_diameter_tolerance_m: f64,
    pub unr_minor_diameter_max_m: f64,
    pub minor_diameter_min_f: f64,
    pub minor_diameter_max_f: f64,
    pub pitch_diameter_min_f: f64,
    pub pitch_diameter_max_f: f64,
    pub pitch_diameter_tolerance_f: f64,
    pub major_diameter_min_f: f64,
}

/// Структура для представления дополнительной информации о резьбе
#[derive(Serialize, Debug)]
pub struct ThreadInfoItem {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Модель данных для ответа клиенту
#[derive(Serialize, Debug)]
pub struct ImperialInfoResponse {
    pub id: i64,
    pub fractional_diameter: String,
    pub decimal_diameter: f64,
    pub description: String,
    pub designation1: String,
    pub designation2: String,
    pub tpi: i32,
    pub pitch: f64,
    pub series_designation: String,
    pub series: String,
    #[serde(rename = "type")]
    pub type_: ThreadType,
    pub thread_depth: f64, // New field for machinists
    pub t_d2: f64,
    pub t_d: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance: Option<f64>,

    pub major_diam_max: f64,
    pub major_diam_es: f64, // major_diam_max - major_diameter_basic
    pub major_diameter_basic: f64,
    pub major_diameter_avg: f64,
    pub major_diam_ei: f64, // major_diam_min - major_diameter_basic
    pub major_diam_min: f64,

    pub pitch_diameter_max: f64,
    pub pitch_diameter_es: f64, // pitch_diameter_max - pitch_diameter_basic
    pub pitch_diameter_basic: f64,
    pub pitch_diameter_ei: f64, // pitch_diameter_min - pitch_diameter_basic
    pub pitch_diameter_min: f64,
    pub pitch_diameter_avg: f64,

    pub minor_diameter_max: f64,
    pub minor_diam_es: f64, // minor_diameter_max - minor_diameter_basic
    pub minor_diameter_basic: f64,
    pub minor_diam_ei: f64, // minor_diameter_min - minor_diameter_basic
    pub minor_diameter_min: f64,

    pub minor_diameter_avg: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unr_minor_diameter_max: Option<f64>,
    // Новые параметры
    pub h: f64,
    pub units: Unit,
    pub pitch_diameter_tolerance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_diam_min2: Option<f64>,
    // Новые поля для отображения допусков (отклонений) относительно базовых значений:
    /// Дополнительная информация о резьбе
    pub additional_info: Vec<ThreadInfoItem>,
}
