use serde::Deserialize;

#[derive(Deserialize)]
pub struct SvgParams {
    #[serde(rename = "type")]
    pub type_: String,
    pub theme: String,
    pub tolerance: String,
    pub diameter: f64,
    pub pitch: f64,
    pub language: String,
    pub units: String,
    pub precision: usize,
    #[serde(default)]
    pub show_dimensions: bool,
}
