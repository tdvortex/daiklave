mod charm;
mod charm_id;
mod error;
mod martial_artist;
mod style;
mod style_id;

pub use charm::MartialArtsCharm;
pub(crate) use charm_id::MartialArtsCharmId;
pub(crate) use error::{
    AddMartialArtsStyleError, RemoveMartialArtsStyleError, SetMartialArtsDotsError,
};

pub(crate) use martial_artist::MartialArtist;
pub use style::MartialArtsStyle;
pub use style_id::MartialArtsStyleId;

use crate::Character;

/// All of the character's Martial Arts styles.
pub struct MartialArts<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> MartialArts<'view, 'source> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, id: MartialArtsStyleId) -> Option<MartialArtist<'view, 'source>> {
        self.0.exalt_state.martial_artist(id)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = MartialArtsStyleId> {
        self.0.exalt_state.martial_arts_id_iter()
    }
}
