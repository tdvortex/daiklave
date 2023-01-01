use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityMemo, exaltation::exalt::martial_arts::ExaltMartialArtistMemo,
    martial_arts::MartialArtsStyle,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtist {
    pub style: MartialArtsStyle,
    pub ability: AbilityMemo,
}

impl From<ExaltMartialArtistMemo> for MortalMartialArtist {
    fn from(exalt_artist: ExaltMartialArtistMemo) -> Self {
        Self {
            style: exalt_artist.style,
            ability: exalt_artist.ability,
        }
    }
}
