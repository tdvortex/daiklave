use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    abilities::Ability,
    exaltation::mortal::martial_arts::MortalMartialArtist,
    martial_arts::{MartialArtsCharm, MartialArtsCharmId, MartialArtsStyle},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtist {
    pub(crate) style: MartialArtsStyle,
    pub(crate) ability: Ability,
    pub(crate) charms: HashMap<MartialArtsCharmId, MartialArtsCharm>,
}

impl From<MortalMartialArtist> for ExaltMartialArtist {
    fn from(mortal_artist: MortalMartialArtist) -> Self {
        Self {
            style: mortal_artist.style,
            ability: mortal_artist.ability,
            charms: HashMap::new(),
        }
    }
}
