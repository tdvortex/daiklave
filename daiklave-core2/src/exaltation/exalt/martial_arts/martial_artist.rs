use crate::{
    abilities::AbilityRating,
    exaltation::mortal::martial_arts::MortalMartialArtistDetails,
    martial_arts::{
        charm::{MartialArtsCharm, MartialArtsCharmDetails},
        style::MartialArtsStyleDetails,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtistDetails<'source> {
    pub(crate) style: &'source MartialArtsStyleDetails,
    pub(crate) ability: AbilityRating<'source>,
    pub(crate) charms: Vec<(&'source str, &'source MartialArtsCharmDetails)>,
}

impl<'view, 'source> ExaltMartialArtistDetails<'source> {
    pub fn style(&'view self) -> &'source MartialArtsStyleDetails {
        self.style
    }

    pub fn ability(&'view self) -> &'view AbilityRating<'source> {
        &self.ability
    }

    pub fn ability_mut(&'view mut self) -> &'view mut AbilityRating<'source> {
        &mut self.ability
    }

    pub fn charms(
        &self,
        style_name: &'source str,
    ) -> impl Iterator<Item = MartialArtsCharm<'source>> + '_ {
        self.charms.iter().map(|(name, details)| MartialArtsCharm {
            name: *name,
            style_name,
            details: *details,
        })
    }
}

impl<'source> From<MortalMartialArtistDetails<'source>> for ExaltMartialArtistDetails<'source> {
    fn from(mortal_artist: MortalMartialArtistDetails<'source>) -> Self {
        Self {
            style: mortal_artist.style(),
            ability: mortal_artist.ability().to_owned(),
            charms: Vec::new(),
        }
    }
}
