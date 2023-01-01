use crate::{
    abilities::AbilityView, exaltation::exalt::martial_arts::ExaltMartialArtistView,
    martial_arts::MartialArtsStyle,
};

use super::MortalMartialArtistMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MortalMartialArtistView<'source> {
    style: &'source MartialArtsStyle,
    ability: AbilityView<'source>,
}

impl<'view, 'source> MortalMartialArtistView<'source> { 
    pub(in crate::exaltation::mortal) fn new(
        style: &'source MartialArtsStyle,
        ability: AbilityView<'source>,
    ) -> Self {
        Self {
            style,
            ability,
        }
    }

    pub fn as_memo(&'view self) -> MortalMartialArtistMemo {
        MortalMartialArtistMemo::new(
            self.style.clone(),
            self.ability.as_memo(),
        )
    }
    
    pub fn style(&'view self) -> &'source MartialArtsStyle {
        self.style
    }

    pub fn ability(&'view self) -> &'view AbilityView<'source> {
        &self.ability
    }

    pub fn ability_mut(&'view mut self) -> &'view mut AbilityView<'source> {
        &mut self.ability
    }
}


impl<'source> From<ExaltMartialArtistView<'source>> for MortalMartialArtistView<'source> {
    fn from(exalt_artist: ExaltMartialArtistView<'source>) -> Self {
        Self {
            style: exalt_artist.style(),
            ability: exalt_artist.ability().to_owned(),
        }
    }
}
