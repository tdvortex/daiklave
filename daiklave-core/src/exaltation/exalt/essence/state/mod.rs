mod memo;
use std::num::NonZeroU8;

pub(crate) use memo::EssenceStateMemo;

use super::motes::MotesState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EssenceState<'source> {
    pub rating: NonZeroU8,
    pub motes: MotesState<'source>,
}

impl<'source> From<&'source EssenceStateMemo> for EssenceState<'source> {
    fn from(value: &'source EssenceStateMemo) -> Self {
        Self {
            rating: value.rating,
            motes: (&value.motes).into(),
        }
    }
}