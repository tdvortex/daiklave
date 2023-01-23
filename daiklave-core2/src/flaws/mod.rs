use crate::Character;

use self::flaw::Flaw;

/// Traits of an individual Flaw.
pub mod flaw;

/// An interface for the Flaws belonging to the character.
pub struct Flaws<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Flaws<'view, 'source> {
    /// Iterates over all Flaws the character possesses by their name.
    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        vec![].into_iter()
    }

    /// Gets a specific Flaw by its name.
    pub fn get(&self, name: &str) -> Option<Flaw<'source>> {
        todo!()
    }
}