mod memo;
pub(crate) use memo::EssenceStateMemo;

use super::motes::MotesState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EssenceState<'source> {
    pub rating: u8,
    pub motes: MotesState<'source>,
}

impl<'source> EssenceState<'source> {
    pub(in crate::exaltation::exalt) fn as_memo(&self) -> EssenceStateMemo {
        EssenceStateMemo::new(self.rating, self.motes.as_memo())
    }
}
