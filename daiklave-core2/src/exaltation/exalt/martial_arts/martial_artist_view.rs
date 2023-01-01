use std::collections::HashMap;

use crate::{
    abilities::AbilityView,
    exaltation::mortal::martial_arts::MortalMartialArtistView,
    martial_arts::{MartialArtsCharm, MartialArtsCharmId, MartialArtsStyle},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtistView<'source> {
    pub(crate) style: &'source MartialArtsStyle,
    pub(crate) ability: AbilityView<'source>,
    pub(crate) charms: HashMap<MartialArtsCharmId, &'source MartialArtsCharm>,
}

impl<'source> From<MortalMartialArtistView<'source>> for ExaltMartialArtistView<'source> {
    fn from(mortal_artist: MortalMartialArtistView<'source>) -> Self {
        Self {
            style: mortal_artist.style,
            ability: mortal_artist.ability,
            charms: HashMap::new(),
        }
    }
}
