use crate::Character;

use self::intimacy::{Intimacy};

/// Details related to a specific Intimacy.
pub mod intimacy;

/// Interface for a character's Intimacies.
pub struct Intimacies<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Intimacies<'view, 'source> {
    /// Iterates over all Intimacies.
    pub fn iter(&self) -> impl Iterator<Item = Intimacy<'source>> + '_ {
        self.0.intimacies.iter().map(|(intimacy_type, intimacy_level)| Intimacy {
            intimacy_type: *intimacy_type,
            level: *intimacy_level
        })
    }
}
