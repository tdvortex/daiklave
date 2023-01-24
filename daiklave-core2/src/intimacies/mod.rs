use crate::Character;

use self::intimacy::{IntimacyId, Intimacy};

/// Details related to a specific Intimacy.
pub mod intimacy;

/// Interface for a character's Intimacies.
pub struct Intimacies<'view, 'source>(&'view Character<'source>);

impl<'view, 'source> Intimacies<'view, 'source> {
    /// Iterates over all Intimacies by their Id.
    pub fn iter(&self) -> impl Iterator<Item = IntimacyId> + '_ {
        self.0.intimacies.keys().copied()
    }

    /// Gets a specific Intimacy if it exists.
    pub fn get(&self, intimacy_id: IntimacyId) -> Option<Intimacy<'source>> {
        self.0.intimacies.get_key_value(&intimacy_id).map(|(id, inner)| Intimacy {
            id: *id,
            inner: *inner
        })
    }
}