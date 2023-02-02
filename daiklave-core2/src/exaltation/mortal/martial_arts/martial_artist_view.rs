use crate::{
    abilities::AbilityRating, exaltation::exalt::martial_arts::ExaltMartialArtistDetails,
    martial_arts::style::MartialArtsStyleDetails,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MortalMartialArtistDetails<'source> {
    pub style: &'source MartialArtsStyleDetails,
    pub ability: AbilityRating<'source>,
}

impl<'view, 'source> MortalMartialArtistDetails<'source> {
    pub fn style(&self) -> &'source MartialArtsStyleDetails {
        self.style
    }

    pub fn ability(&'view self) -> &'view AbilityRating<'source> {
        &self.ability
    }

    pub fn ability_mut(&'view mut self) -> &'view mut AbilityRating<'source> {
        &mut self.ability
    }
}

impl<'source> From<ExaltMartialArtistDetails<'source>> for MortalMartialArtistDetails<'source> {
    fn from(exalt_artist: ExaltMartialArtistDetails<'source>) -> Self {
        Self {
            style: exalt_artist.style(),
            ability: exalt_artist.ability().to_owned(),
        }
    }
}
