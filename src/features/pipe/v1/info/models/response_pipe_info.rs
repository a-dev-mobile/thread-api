use serde::Serialize;

use crate::features::pipe::v1::common::models::ModelPipeAdditionalInfo;
use crate::features::pipe::v1::common::models::ModelPipeDiameterInfo;

// Response models
#[derive(Debug, Serialize)]
pub struct ResponsePipeInfo {
    pub designation1: String,
    pub designation2: String,
    pub unit: String,
    pub description: String,
    pub main_info: Vec<ModelPipeAdditionalInfo>,
    pub diameter_info: Vec<ModelPipeDiameterInfo>,
    pub additional_info: Vec<ModelPipeAdditionalInfo>,
}
