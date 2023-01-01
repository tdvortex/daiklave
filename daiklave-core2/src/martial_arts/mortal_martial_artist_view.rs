use crate::abilities::AbilityView;

use super::{exalt_martial_artist_view::ExaltMartialArtistView, style::MartialArtsStyle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MortalMartialArtistView<'source> {
    pub style: &'source MartialArtsStyle,
    pub ability: AbilityView<'source>,
}

impl<'source> From<ExaltMartialArtistView<'source>> for MortalMartialArtistView<'source> {
    fn from(exalt_artist: ExaltMartialArtistView<'source>) -> Self {
        Self {
            style: exalt_artist.style,
            ability: exalt_artist.ability,
        }
    }
}
