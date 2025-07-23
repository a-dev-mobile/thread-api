use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow,Clone)]
pub struct ModelPipeDB {
    pub id: i32,
    pub designation: f64,
    pub designation_2: String,
    pub thread_pitch: f64,
    pub thread_per: i32,
    pub class_name: Option<String>,
    // External thread dimensions
    pub ex_major_dia_max: Option<f64>,
    pub ex_major_dia_min: Option<f64>,
    pub ex_pitch_diam_max: Option<f64>,
    pub ex_pitch_diam_min: Option<f64>,
    pub ex_minor_dia_max: Option<f64>,
    // Internal thread dimensions
    pub in_minor_dia_min: Option<f64>,
    pub in_minor_dia_max: Option<f64>,
    pub in_pitch_diam_min: Option<f64>,
    pub in_pitch_diam_max: Option<f64>,
    pub in_major_dia_min: Option<f64>,
    pub in_tap_drill: Option<f64>,
}