use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

/// Определяет тип резьбы: наружная или внутренняя
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ThreadType {
    /// Наружная резьба
    Male,
    /// Внутренняя резьба
    Female,
}
