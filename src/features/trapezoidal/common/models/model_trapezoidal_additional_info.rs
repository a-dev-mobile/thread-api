use serde::Serialize;

use crate::features::trapezoidal::common::enums::TypeTrapezoidalAdditionalInfo;

#[derive(Debug, Serialize)]
pub struct ModelTrapezoidalAdditionalInfo {
    #[serde(skip)]
    pub type_trapezoidal_additional_info: Option<TypeTrapezoidalAdditionalInfo>,
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
