use serde::{Deserialize, Serialize};

use crate::abilities::Ability;

use super::{exalt_martial_artist::ExaltMartialArtist, style::MartialArtsStyle};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtist {
    pub style: MartialArtsStyle,
    pub ability: Ability,
}

impl From<ExaltMartialArtist> for MortalMartialArtist {
    fn from(exalt_artist: ExaltMartialArtist) -> Self {
        Self {
            style: exalt_artist.style,
            ability: exalt_artist.ability,
        }
    }
}
