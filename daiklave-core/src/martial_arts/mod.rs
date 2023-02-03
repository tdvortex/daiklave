/// Details of an individual Martial Arts Charm.
pub mod charm;
mod error;
mod martial_artist;

/// Details of an individual Martial Arts style.
pub mod style;

pub(crate) use error::MartialArtsError;

pub use martial_artist::MartialArtsStyle;

use crate::exaltation::Exaltation;

/// All of the character's Martial Arts styles.
pub struct MartialArts<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> MartialArts<'view, 'source> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, name: &str) -> Option<MartialArtsStyle<'view, 'source>> {
        self.0.martial_artist(name)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = &'source str> {
        self.0.martial_arts_id_iter()
    }
}
