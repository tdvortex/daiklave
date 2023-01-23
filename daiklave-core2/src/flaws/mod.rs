use crate::Character;

use self::flaw::Flaw;

/// Traits of an individual Flaw.
pub mod flaw;

/// An interface for the Flaws belonging to the character.
pub struct Flaws<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Flaws<'view, 'source> {
    /// Iterates over all Flaws the character possesses by their name.
    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.0.flaws.iter().map(|(&name, _)| name)
    }

    /// Gets a specific Flaw by its name.
    pub fn get(&self, name: &str) -> Option<Flaw<'source>> {
        self.0.flaws.get_key_value(name).map(|(&name, &(book_reference, description))| Flaw {
            name,
            book_reference,
            description
        })
    }
}