use std::ops::Deref;

use super::Hearthstone;

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