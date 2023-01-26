use crate::{
    abilities::AbilityRating, exaltation::exalt::martial_arts::ExaltMartialArtist,
    martial_arts::MartialArtsStyle,
};

use super::MortalMartialArtistMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MortalMartialArtist<'source> {
    pub style: &'source MartialArtsStyle,
    pub ability: AbilityRating<'source>,
}

impl<'view, 'source> MortalMartialArtist<'source> {
    pub(in crate::exaltation::mortal) fn new(
        style: &'source MartialArtsStyle,
        ability: AbilityRating<'source>,
    ) -> Self {
        Self { style, ability }
    }

    pub fn as_memo(&'view self) -> MortalMartialArtistMemo {
        MortalMartialArtistMemo::new(self.style.clone(), self.ability.as_memo())
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
}

impl<'source> From<ExaltMartialArtist<'source>> for MortalMartialArtist<'source> {
    fn from(exalt_artist: ExaltMartialArtist<'source>) -> Self {
        Self {
            style: exalt_artist.style(),
            ability: exalt_artist.ability().to_owned(),
        }
    }
}
