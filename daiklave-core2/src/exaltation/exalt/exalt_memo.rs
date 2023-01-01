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
    martial_arts::ExaltMartialArtistMemo, 
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMemo {
    essence: EssenceMemo,
    martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistMemo>,
    exalt_type: ExaltTypeMemo,
}