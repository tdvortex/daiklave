use serde::{Deserialize, Serialize};

use super::{solar::SolarMemo, ExaltType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltTypeMemo {
    Solar(SolarMemo),
}

impl<'source> ExaltTypeMemo {
    pub fn as_ref(&'source self) -> ExaltType<'source> {
        match self {
            ExaltTypeMemo::Solar(memo) => ExaltType::Solar(memo.as_ref()),
        }
    }
}
