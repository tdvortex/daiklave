use crate::Character;

use super::{style_id::MartialArtsStyleId, MartialArtist};

/// All of the character's Martial Arts styles.
pub struct MartialArts<'char>(&'char Character);

impl<'char> MartialArts<'char> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, id: MartialArtsStyleId) -> Option<MartialArtist<'char>> {
        self.0.exalt_state.martial_artist(id)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = MartialArtsStyleId> {
        self.0.exalt_state.martial_arts_id_iter()
    }
}
