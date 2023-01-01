use serde::{Serialize, Deserialize};

use super::solar::SolarMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltTypeMemo {
    Solar(SolarMemo),
}

impl ExaltTypeMemo {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&SolarMemo> {
        match self {
            ExaltTypeMemo::Solar(solar_traits) => Some(solar_traits),
        }
    }
}