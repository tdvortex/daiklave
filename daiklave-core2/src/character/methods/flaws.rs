use std::collections::hash_map::Entry;

use crate::{
    flaws::{flaw::AddFlaw, Flaws},
    merits::merit_new::MeritError,
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Gets the Flaws the character possesses.
    pub fn flaws(&'view self) -> Flaws<'view, 'source> {
        Flaws(self)
    }

    /// Adds a Flaw to the character.
    pub fn add_flaw(
        &mut self,
        flaw: &'source AddFlaw,
    ) -> Result<&mut Self, CharacterMutationError> {
        let AddFlaw {
            name,
            book_reference,
            description,
        } = flaw;

        if let Entry::Vacant(e) = self.flaws.entry(name.as_str()) {
            e.insert((*book_reference, description.as_str()));
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        }
    }

    /// Removes a Flaw from the Character.
    pub fn remove_flaw(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.flaws.remove(name).is_some() {
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(MeritError::NotFound))
        }
    }
}
