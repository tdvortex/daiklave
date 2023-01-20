use std::collections::hash_map::Entry;

use crate::{hearthstones::hearthstone::GeomancyLevel, unique_id::UniqueId, CharacterMutationError, Character, merits::merit::MeritError};

impl<'source> Character<'source> {
    /// Adds a demense to the character's merits list.
    pub fn add_demense(&mut self, demense_id: UniqueId, name: &'source str, geomancy_level: GeomancyLevel) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.demenses_no_manse.entry(demense_id) {
            e.insert((name, geomancy_level));
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(MeritError::DuplicateMerit))
        }
    }

    /// Removes a demense from a character's merits list.
    pub fn remove_demense(&mut self, demense_id: UniqueId) -> Result<&mut Self, CharacterMutationError> {
        if self.demenses_no_manse.remove(&demense_id).is_none() {
            Err(CharacterMutationError::MeritError(MeritError::NotFound))
        } else {
            Ok(self)
        }
    }
}