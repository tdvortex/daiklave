use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LimitMemo {
    pub track: u8,
    pub trigger: String,
}