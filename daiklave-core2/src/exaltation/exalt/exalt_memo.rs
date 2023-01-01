use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    martial_arts::{ MartialArtsStyleId,
    },
};

use super::{
    essence::{EssenceMemo,
    },
    exalt_type::{ExaltTypeMemo},
    martial_arts::ExaltMartialArtistMemo, ExaltView, 
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMemo {
    essence: EssenceMemo,
    martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistMemo>,
    exalt_type: ExaltTypeMemo,
}

impl<'source> ExaltMemo {
    pub(in crate::exaltation::exalt) fn new(
        essence: EssenceMemo,
        martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistMemo>,
        exalt_type: ExaltTypeMemo,
    ) -> Self {
        Self {
            essence,
            martial_arts_styles,
            exalt_type,
        }
    }

    pub fn as_ref(&'source self) -> ExaltView<'source> {
        ExaltView::new(self.essence.as_ref(), self.martial_arts_styles.iter().map(|(k, v)| (*k, v.as_ref())).collect(), self.exalt_type.as_ref())
    }
}