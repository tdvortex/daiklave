mod memo;
use std::num::NonZeroU8;

pub(crate) use memo::EssenceStateMemo;

use super::motes::MotesState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EssenceState<'source> {
    pub rating: NonZeroU8,
    pub motes: MotesState<'source>,
}

impl<'source> EssenceState<'source> {
    pub(in crate::exaltation::exalt) fn as_memo(&self) -> EssenceStateMemo {
        EssenceStateMemo {
            rating: self.rating,
            motes: self.motes.as_memo(),
        }
    }
}
