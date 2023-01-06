use serde::{Deserialize, Serialize};

use crate::hearthstone::memo::HearthstoneMemo;

use super::OwnedHearthstone;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OwnedHearthstoneMemo {
    pub hearthstone: HearthstoneMemo,
    pub manse: Option<String>,
}

impl<'source> OwnedHearthstoneMemo {
    pub fn as_ref(&'source self) -> OwnedHearthstone<'source> {
        OwnedHearthstone {
            hearthstone: self.hearthstone.as_ref(),
            manse: self.manse.as_deref(),
        }
    }
}
