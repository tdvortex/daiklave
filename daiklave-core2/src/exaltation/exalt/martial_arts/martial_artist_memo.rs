use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityRatingMemo,
    martial_arts::{
        charm::{MartialArtsCharm, MartialArtsCharmId},
        MartialArtsStyle,
    },
};

use super::ExaltMartialArtist;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtistMemo {
    pub style: MartialArtsStyle,
    pub ability: AbilityRatingMemo,
    pub charms: Vec<(MartialArtsCharmId, MartialArtsCharm)>,
}

impl<'source> ExaltMartialArtistMemo {
    pub fn as_ref(&'source self) -> ExaltMartialArtist<'source> {
        ExaltMartialArtist {
            style: &self.style,
            ability: self.ability.as_ref(),
            charms: self.charms.iter().map(|(id, charm)| (*id, charm)).collect(),
        }
    }
}
