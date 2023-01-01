use crate::CharacterView;

use super::{martial_artist_view::MartialArtistView, MartialArtsStyleId};

/// All of the character's Martial Arts styles.
pub struct MartialArtsView<'view, 'source>(pub(crate) &'view CharacterView<'source>);

impl<'view, 'source> MartialArtsView<'view, 'source> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, id: MartialArtsStyleId) -> Option<MartialArtistView<'view, 'source>> {
        self.0.exalt_state.martial_artist(id)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = MartialArtsStyleId> {
        self.0.exalt_state.martial_arts_id_iter()
    }
}
