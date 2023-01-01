use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityMemo, martial_arts::MartialArtsStyle};

use super::MortalMartialArtistView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtistMemo {
    style: MartialArtsStyle,
    ability: AbilityMemo,
}

impl<'source> MortalMartialArtistMemo {
    pub(in crate::exaltation::mortal::martial_arts) fn new(
        style: MartialArtsStyle,
        ability: AbilityMemo,
    ) -> Self {
        Self { style, ability }
    }

    pub fn as_ref(&'source self) -> MortalMartialArtistView<'source> {
        MortalMartialArtistView::new(&self.style, self.ability.as_ref())
    }
}
