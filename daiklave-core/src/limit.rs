use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Limit {
    pub track: u8,
    pub limit_trigger: String,
}
