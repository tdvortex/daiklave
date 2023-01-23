use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::motes::MotesStateMemo;

use super::EssenceState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EssenceStateMemo {
    pub rating: NonZeroU8,
    pub motes: MotesStateMemo,
}

impl<'source> EssenceStateMemo {
    pub fn as_ref(&'source self) -> EssenceState<'source> {
        EssenceState {
            rating: self.rating,
            motes: self.motes.as_ref(),
        }
    }
}
