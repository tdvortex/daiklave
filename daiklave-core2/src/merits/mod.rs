pub mod merit;

use crate::Character;

use self::merit::{MeritId, Merit};

/// The merits possessed by a character.
pub struct Merits<'view, 'source>(&'view Character<'source>);

impl<'view, 'source> Merits<'view, 'source> {
    /// Gets a specific Merit belonging to the character (if it exists).
    pub fn get(&self, merit_id: MeritId) -> Option<Merit<'source>> {
        todo!()
    }

    /// Iterates over all Merits owned by the character by their Id.
    pub fn iter(&self) -> impl Iterator<Item = MeritId> + '_ {
        vec![].into_iter()
    }
}