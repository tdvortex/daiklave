use super::Hearthstone;

mod memo;
pub(crate) use memo::OwnedHearthstoneMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedHearthstone<'source> {
    hearthstone: Hearthstone<'source>,
    manse: Option<&'source str>,
}

impl<'source> OwnedHearthstone<'source> {
    pub(crate) fn as_memo(&'source self) -> OwnedHearthstoneMemo {
        OwnedHearthstoneMemo {
            hearthstone: self.hearthstone.as_memo(),
            manse: self.manse.map(|s| s.to_string()),
        }
    }
}
