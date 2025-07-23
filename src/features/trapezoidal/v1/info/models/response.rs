use serde::Serialize;

use crate::features::trapezoidal::common::models::ModelTrapezoidalAdditionalInfo;
use crate::features::trapezoidal::common::models::ModelTrapezoidalDiameterInfo;

// Response models
#[derive(Debug, Serialize)]
pub struct ResponseTrapezoidalInfo {
    pub description: String,
    pub designation: String,
    pub main_info: Vec<ModelTrapezoidalAdditionalInfo>,
    pub diameter_info: Vec<ModelTrapezoidalDiameterInfo>,
    pub additional_info: Vec<ModelTrapezoidalAdditionalInfo>,
}
