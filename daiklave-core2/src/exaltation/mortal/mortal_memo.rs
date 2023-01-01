use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    martial_arts::{MartialArtsStyleId,
    },
    sorcery::circles::terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo,
};

use super::martial_arts::MortalMartialArtistMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct MortalMemo {
    martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistMemo>,
    sorcery: Option<TerrestrialCircleSorcererMemo>,
}