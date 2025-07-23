use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TypeTrapezoidalDiameter {
    Major,
    Pitch,
    Minor,
}
