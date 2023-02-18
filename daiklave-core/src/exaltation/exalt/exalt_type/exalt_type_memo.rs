use serde::{Deserialize, Serialize};

use super::{solar::SolarMemo, ExaltType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltTypeMemo {
    Solar(SolarMemo),
}

impl From<&ExaltType<'_>> for ExaltTypeMemo {
    fn from(value: &ExaltType<'_>) -> Self {
        match value {
            ExaltType::Solar(solar) => Self::Solar(solar.into())
        }
    }
}