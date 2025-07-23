use serde::Serialize;



#[derive(Serialize)]
pub struct ModelPipeDiameter {
    pub id: i32,
    pub fractional: String,
    pub decimal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<String>,
}

#[derive(Serialize)]
pub struct ResponsePipeDiameters {
    pub male: Vec<ModelPipeDiameter>,
    pub female: Vec<ModelPipeDiameter>,
}