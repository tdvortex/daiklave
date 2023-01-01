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
    pub fn as_ref(&'source self) -> ExaltView<'source> {
        ExaltView {
            essence: self.essence.as_ref(),
            martial_arts_styles: self.martial_arts_styles.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            exalt_type: self.exalt_type.as_ref(),
        }
    }
}