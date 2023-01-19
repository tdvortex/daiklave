use std::collections::hash_map::Entry;

use crate::{hearthstones::hearthstone::GeomancyLevel, unique_id::UniqueId, CharacterMutationError, Character, merits::merit::MeritError};

impl<'source> Character<'source> {
    /// Checks if a demense (without a manse) can be added to the character.
    pub fn check_add_demense(&self, demense_id: UniqueId, _name: &'source str, _geomancy_level: GeomancyLevel) -> Result<(), CharacterMutationError> {
        if self.demenses_no_manse.contains_key(&demense_id) {
            Err(CharacterMutationError::MeritError(MeritError::DuplicateMerit))
        } else {
            Ok(())
        }
    }

    /// Adds a demense to the character's merits list.
    pub fn add_demense(&mut self, demense_id: UniqueId, name: &'source str, geomancy_level: GeomancyLevel) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.demenses_no_manse.entry(demense_id) {
            e.insert((name, geomancy_level));
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(MeritError::DuplicateMerit))
        }
    }

    /// Checks if a demense can be removed from the character's merits list.
    pub fn check_remove_demense(&self, demense_id: UniqueId) -> Result<(), CharacterMutationError> {
        if self.demenses_no_manse.contains_key(&demense_id) {
            Ok(())
        } else {
            Err(CharacterMutationError::MeritError(MeritError::NotFound))
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