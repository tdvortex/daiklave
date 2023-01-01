use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityMemo,
    martial_arts::MartialArtsStyle,
};

use super::MortalMartialArtistView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtistMemo {
    style: MartialArtsStyle,
    ability: AbilityMemo,
}

impl<'source> MortalMartialArtistMemo {
    pub fn as_ref(&'source self) -> MortalMartialArtistView<'source> {
        MortalMartialArtistView {
            style: &self.style,
            ability: self.ability.as_ref(),
        }
    }
}