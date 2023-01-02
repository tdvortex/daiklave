use std::collections::HashMap;

use crate::{
    abilities::AbilityRating,
    exaltation::mortal::martial_arts::MortalMartialArtistView,
    martial_arts::{MartialArtsCharm, MartialArtsCharmId, MartialArtsStyle},
};

use super::ExaltMartialArtistMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtist<'source> {
    style: &'source MartialArtsStyle,
    ability: AbilityRating<'source>,
    charms: HashMap<MartialArtsCharmId, &'source MartialArtsCharm>,
}

impl<'view, 'source> ExaltMartialArtist<'source> {
    pub fn new(
        style: &'source MartialArtsStyle,
        ability: AbilityRating<'source>,
        charms: HashMap<MartialArtsCharmId, &'source MartialArtsCharm>,
    ) -> Self {
        Self {
            style,
            ability,
            charms,
        }
    }

    pub fn as_memo(&'view self) -> ExaltMartialArtistMemo {
        ExaltMartialArtistMemo::new(
            self.style.to_owned(),
            self.ability.as_memo(),
            self.charms
                .iter()
                .map(|(k, v)| (*k, (*v).to_owned()))
                .collect(),
        )
    }

    pub fn style(&'view self) -> &'source MartialArtsStyle {
        self.style
    }

    pub fn ability(&'view self) -> &'view AbilityRating<'source> {
        &self.ability
    }

    pub fn ability_mut(&'view mut self) -> &'view mut AbilityRating<'source> {
        &mut self.ability
    }

    pub fn charms(
        &'view self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
        self.charms.iter().map(|(k, v)| (*k, *v))
    }

    pub fn charms_mut(
        &'view mut self,
    ) -> &'view mut HashMap<MartialArtsCharmId, &'source MartialArtsCharm> {
        &mut self.charms
    }
}

impl<'source> From<MortalMartialArtistView<'source>> for ExaltMartialArtist<'source> {
    fn from(mortal_artist: MortalMartialArtistView<'source>) -> Self {
        Self {
            style: mortal_artist.style(),
            ability: mortal_artist.ability().to_owned(),
            charms: HashMap::new(),
        }
    }
}
