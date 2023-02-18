use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityRatingMemo, martial_arts::style::MartialArtsStyleDetails};

use super::MortalMartialArtistDetails;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtistDetailsMemo {
    pub style: MartialArtsStyleDetails,
    pub ability: AbilityRatingMemo,
}

impl From<&MortalMartialArtistDetails<'_>> for MortalMartialArtistDetailsMemo {
    fn from(value: &MortalMartialArtistDetails<'_>) -> Self {
        Self {
            style: value.style.to_owned(),
            ability: (&value.ability).into(),
        }
    }
}