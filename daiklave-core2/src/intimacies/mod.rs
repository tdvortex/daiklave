use crate::Character;

use self::intimacy::{IntimacyId, Intimacy};

/// Details related to a specific Intimacy.
pub mod intimacy;

/// Interface for a character's Intimacies.
pub struct Intimacies<'view, 'source>(&'view Character<'source>);

impl<'view, 'source> Intimacies<'view, 'source> {
    /// Iterates over all Intimacies by their Id.
    pub fn iter(&self) -> impl Iterator<Item = IntimacyId> + '_ {
        vec![].into_iter()
    }

    /// Gets a specific Intimacy if it exists.
    pub fn get(&self, intimacy_id: IntimacyId) -> Option<Intimacy<'source>> {
        todo!()
    }
}