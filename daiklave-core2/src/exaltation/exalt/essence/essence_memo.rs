use serde::{Deserialize, Serialize};

use super::{motes_memo::MotesMemo, Essence};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EssenceMemo {
    rating: u8,
    motes: MotesMemo,
}

impl<'source> EssenceMemo {
    pub fn new(rating: u8, motes: MotesMemo) -> Self {
        Self { rating, motes }
    }

    pub fn as_ref(&'source self) -> Essence<'source> {
        Essence {
            rating: self.rating,
            motes: self.motes.as_ref(),
        }
    }
}
