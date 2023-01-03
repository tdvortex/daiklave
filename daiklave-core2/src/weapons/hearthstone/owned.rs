use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::{Hearthstone, HearthstoneMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedHearthstone<'source> {
    hearthstone: Hearthstone<'source>,
    manse: Option<&'source str>,
}

impl<'source> Deref for OwnedHearthstone<'source> {
    type Target = Hearthstone<'source>;

    fn deref(&self) -> &Self::Target {
        &self.hearthstone
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