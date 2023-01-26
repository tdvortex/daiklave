use crate::{
    abilities::AbilityRating,
    exaltation::mortal::martial_arts::MortalMartialArtist,
    martial_arts::{
        charm::{MartialArtsCharm, MartialArtsCharmId},
        style::MartialArtsStyle,
    },
};

use super::ExaltMartialArtistMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtist<'source> {
    pub(crate) style: &'source MartialArtsStyle,
    pub(crate) ability: AbilityRating<'source>,
    pub(crate) charms: Vec<(MartialArtsCharmId, &'source MartialArtsCharm)>,
}

impl<'view, 'source> ExaltMartialArtist<'source> {
    pub fn as_memo(&'view self) -> ExaltMartialArtistMemo {
        ExaltMartialArtistMemo {
            style: self.style.to_owned(),
            ability: self.ability.as_memo(),
            charms: self
                .charms
                .iter()
                .map(|(charm_id, charm)| (*charm_id, (*charm).to_owned()))
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
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
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
