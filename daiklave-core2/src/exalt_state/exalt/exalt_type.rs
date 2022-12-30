use serde::{Deserialize, Serialize};

pub mod solar;
use solar::{Solar, SolarView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltType {
    Solar(Solar),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltTypeView {
    Solar(SolarView),
}
