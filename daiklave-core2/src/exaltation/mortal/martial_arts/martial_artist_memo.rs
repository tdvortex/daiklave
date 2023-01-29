use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityRatingMemo, martial_arts::style::MartialArtsStyle};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtistMemo {
    pub style: MartialArtsStyle,
    pub ability: AbilityRatingMemo,
}