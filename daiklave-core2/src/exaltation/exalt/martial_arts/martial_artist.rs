use crate::{
    abilities::AbilityRating,
    exaltation::mortal::martial_arts::MortalMartialArtist,
    martial_arts::{charm::MartialArtsCharmDetails, style::MartialArtsStyle},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtist<'source> {
    pub(crate) style: &'source MartialArtsStyle,
    pub(crate) ability: AbilityRating<'source>,
    pub(crate) charms: Vec<(&'source str, &'source MartialArtsCharmDetails)>,
}

impl<'view, 'source> ExaltMartialArtist<'source> {
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
    ) -> impl Iterator<Item = (&'source str, &'source MartialArtsCharmDetails)> + '_ {
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
