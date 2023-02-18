use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::motes::MotesStateMemo;

use super::EssenceState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EssenceStateMemo {
    pub rating: NonZeroU8,
    pub motes: MotesStateMemo,
}

impl From<&EssenceState<'_>> for EssenceStateMemo {
    fn from(value: &EssenceState<'_>) -> Self {
        Self {
            rating: value.rating,
            motes: (&value.motes).into(),
        }
    }
}