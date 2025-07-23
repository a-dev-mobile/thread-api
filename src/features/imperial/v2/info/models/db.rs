use serde::Serialize;
use sqlx::FromRow;


#[derive(Serialize, FromRow, Debug, Clone)]
pub struct ModelV2ImperialDB {
    pub id: i64,
    pub diameter: String,
    pub diameter_2: f64,
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