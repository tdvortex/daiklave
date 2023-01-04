use serde::{Serialize, Deserialize};

use super::{Hearthstone, HearthstoneMemo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedHearthstone<'source> {
    hearthstone: Hearthstone<'source>,
    manse: Option<&'source str>,
}

impl<'source> OwnedHearthstone<'source> {
    pub fn as_memo(&'source self) -> OwnedHearthstoneMemo {
        OwnedHearthstoneMemo { hearthstone: self.hearthstone.as_memo(), manse: self.manse.map(|s| s.to_string()) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedHearthstoneMemo {
    hearthstone: HearthstoneMemo,
    manse: Option<String>,
}

impl<'source> OwnedHearthstoneMemo {
    pub fn as_ref(&'source self) -> OwnedHearthstone<'source> {
        OwnedHearthstone { hearthstone: self.hearthstone.as_ref(), manse: self.manse.as_deref() }
    }
}