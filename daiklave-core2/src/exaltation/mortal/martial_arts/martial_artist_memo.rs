use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityRatingMemo, martial_arts::style::MartialArtsStyle};

use super::MortalMartialArtist;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtistMemo {
    style: MartialArtsStyle,
    ability: AbilityRatingMemo,
}

impl<'source> MortalMartialArtistMemo {
    pub(in crate::exaltation::mortal::martial_arts) fn new(
        style: MartialArtsStyle,
        ability: AbilityRatingMemo,
    ) -> Self {
        Self { style, ability }
    }

    pub fn as_ref(&'source self) -> MortalMartialArtist<'source> {
        MortalMartialArtist::new(&self.style, self.ability.as_ref())
    }
}
