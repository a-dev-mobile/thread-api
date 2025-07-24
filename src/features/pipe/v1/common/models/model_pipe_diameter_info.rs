use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct ModelPipeDiameterInfo {
    pub name: String,
    pub max: String,
    pub es: String,
    pub basic: String,
    pub avg: String,
    pub ei: String,
    pub min: String,
}
