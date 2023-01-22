/// Details of an individual Martial Arts Charm.
pub mod charm;
mod error;
mod martial_artist;
mod style;
mod style_id;

pub(crate) use error::MartialArtsError;

pub use martial_artist::MartialArtist;
pub use style::MartialArtsStyle;
pub use style_id::MartialArtsStyleId;

use crate::exaltation::Exaltation;

/// All of the character's Martial Arts styles.
pub struct MartialArts<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> MartialArts<'view, 'source> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, id: MartialArtsStyleId) -> Option<MartialArtist<'view, 'source>> {
        self.0.martial_artist(id)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = MartialArtsStyleId> {
        self.0.martial_arts_id_iter()
    }
}
