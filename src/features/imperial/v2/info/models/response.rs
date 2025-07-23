use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseV2ImperialInfo {
    pub designation1: String,
    pub designation2: String,
    pub unit: String,
    pub description: String,
    pub main_info: Vec<ModelImperialAdditionalInfo>,
    pub diameter_info: Vec<ModelImperialDiameterInfo>,
    pub additional_info: Vec<ModelImperialAdditionalInfo>,
}

#[derive(Debug, Serialize)]
pub struct ModelImperialAdditionalInfo {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct ModelImperialDiameterInfo {
    pub name: String,
    pub max: String,
    pub es: String,
    pub basic: String,
    pub avg: String,
    pub ei: String,
    pub min: String,
}
