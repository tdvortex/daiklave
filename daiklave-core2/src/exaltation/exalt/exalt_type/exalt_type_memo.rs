use serde::{Serialize, Deserialize};

use super::{solar::SolarMemo, ExaltTypeView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltTypeMemo {
    Solar(SolarMemo),
}

impl<'source> ExaltTypeMemo {
    pub fn as_ref(&'source self) -> ExaltTypeView<'source> {
        match self {
            ExaltTypeMemo::Solar(memo) => ExaltTypeView::Solar(memo.as_ref()),
        }
    }
}