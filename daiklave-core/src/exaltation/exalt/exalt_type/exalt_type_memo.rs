use serde::{Deserialize, Serialize};

use super::solar::SolarMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltTypeMemo {
    Solar(SolarMemo),
}
