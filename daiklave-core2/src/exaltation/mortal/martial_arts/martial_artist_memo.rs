use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityRatingMemo, martial_arts::style::MartialArtsStyleDetails};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtistMemo {
    pub style: MartialArtsStyleDetails,
    pub ability: AbilityRatingMemo,
}