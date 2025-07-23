use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum TypePipeAdditionalInfo {
    ac,
    p,
    H1,
    H4_h3,
    r1_max,
    r2_max,
    z,
}
