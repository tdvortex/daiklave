use serde::{Deserialize, Serialize};

use super::{motes_memo::MotesMemo, EssenceView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EssenceMemo {
    rating: u8,
    motes: MotesMemo,
}

impl<'source> EssenceMemo {
    pub fn as_ref(&'source self) -> EssenceView<'source> {
        EssenceView { 
            rating: self.rating, 
            motes: self.motes.as_ref(), 
        }
    }
}