use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::essence::motes::MotesStateMemo;

use super::EssenceState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EssenceStateMemo {
    rating: u8,
    motes: MotesStateMemo,
}

impl<'source> EssenceStateMemo {
    pub fn new(rating: u8, motes: MotesStateMemo) -> Self {
        Self { rating, motes }
    }

    pub fn as_ref(&'source self) -> EssenceState<'source> {
        EssenceState {
            rating: self.rating,
            motes: self.motes.as_ref(),
        }
    }
}