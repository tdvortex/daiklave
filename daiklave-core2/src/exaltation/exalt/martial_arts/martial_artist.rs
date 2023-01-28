use crate::{
    abilities::AbilityRating,
    exaltation::mortal::martial_arts::MortalMartialArtist,
    martial_arts::{
        charm::{MartialArtsCharm},
        style::MartialArtsStyle,
    },
};

use super::ExaltMartialArtistMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtist<'source> {
    pub(crate) style: &'source MartialArtsStyle,
    pub(crate) ability: AbilityRating<'source>,
    pub(crate) charms: Vec<(&'source str, &'source MartialArtsCharm)>,
}

impl<'view, 'source> ExaltMartialArtist<'source> {
    pub fn as_memo(&'view self) -> ExaltMartialArtistMemo {
        ExaltMartialArtistMemo {
            style: self.style.to_owned(),
            ability: self.ability.as_memo(),
            charms: self
                .charms
                .iter()
                .map(|(charm_name, charm)| ((*charm_name).to_owned(), (*charm).to_owned()))
                .collect(),
        }
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
    ) -> impl Iterator<Item = (&'source str, &'source MartialArtsCharm)> + '_ {
        self.charms.iter().map(|(k, v)| (*k, *v))
    }
}

impl<'source> From<MortalMartialArtist<'source>> for ExaltMartialArtist<'source> {
    fn from(mortal_artist: MortalMartialArtist<'source>) -> Self {
        Self {
            style: mortal_artist.style(),
            ability: mortal_artist.ability().to_owned(),
            charms: Vec::new(),
        }
    }
}
