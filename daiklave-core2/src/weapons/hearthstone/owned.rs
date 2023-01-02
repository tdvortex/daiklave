use super::Hearthstone;

pub(in crate::weapons) struct OwnedHearthstone<'source> {
    hearthstone: Hearthstone<'source>,
    manse: Option<&'source str>,
}