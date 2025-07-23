use serde::Serialize;

use crate::features::pipe::v1::common::enums::TypePipeAdditionalInfo;

#[derive(Debug, Serialize)]
pub struct ModelPipeAdditionalInfo {

    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
