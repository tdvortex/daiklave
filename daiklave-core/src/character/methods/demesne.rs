use std::collections::hash_map::Entry;

use crate::{
    merits::merit::{AddDemense, MeritError},
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Adds a demense to the character's merits list.
    pub fn add_demense(
        &mut self,
        add_demense: &'source AddDemense,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.demenses_no_manse.entry(&add_demense.name) {
            e.insert(add_demense.geomancy_level);
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        }
    }

    /// Removes a demense from a character's merits list.
    pub fn remove_demense(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.demenses_no_manse.remove(name).is_none() {
            Err(CharacterMutationError::MeritError(MeritError::NotFound))
        } else {
            Ok(self)
        }
    }
}
