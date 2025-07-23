use serde::Serialize;

use crate::features::trapezoidal::common::enums::TypeTrapezoidalDiameter;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct ModelTrapezoidalDiameterInfo {
    #[serde(skip)]
    pub type_trapezoidal_diameter: Option<TypeTrapezoidalDiameter>,
    pub name: String,
    pub max: String,
    pub es: String,
    pub basic: String,
    pub avg: String,
    pub ei: String,
    pub min: String,
}
