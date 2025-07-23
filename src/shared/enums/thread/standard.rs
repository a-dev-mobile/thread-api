use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

/// Стандарт резьбы
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ThreadStandard {
    /// Метрическая резьба
    Metric,
    /// Трапецеидальная резьба
    Trapezoidal,
    /// Дюймовая резьба
    Imperial,
    /// Трубная резьба
    Pipe,
}
