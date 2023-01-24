use std::collections::hash_map::Entry;

use crate::{Character, intimacies::{Intimacies, intimacy::{IntimacyMutation, IntimacyLevel, IntimacyError, IntimacyId}}, CharacterMutationError};

impl<'view, 'source> Character<'source> {
    /// The Intimacies a character holds an emotional connection to.
    pub fn intimacies(&'view self) -> Intimacies<'view, 'source> {
        Intimacies(self)
    }

    /// Adds a new Intimacy to a character.
    pub fn add_intimacy(&mut self, intimacy: &'source IntimacyMutation) -> Result<&mut Self, CharacterMutationError> {
        let IntimacyMutation {
            id,
            inner,
        } = intimacy;

        if let Entry::Vacant(e) = self.intimacies.entry(*id) {
            e.insert(inner.as_ref());
            Ok(self)
        } else {
            Err(CharacterMutationError::IntimacyError(IntimacyError::DuplicateIntimacy))
        }
    }

    /// Removes an Intimacy from the character.
    pub fn remove_intimacy(&mut self, id: IntimacyId) -> Result<&mut Self, CharacterMutationError> {
        if self.intimacies.remove(&id).is_some() {
            Ok(self)
        } else {
            Err(CharacterMutationError::IntimacyError(IntimacyError::NotFound))
        }
    }

    /// Sets the Intimacy level of an existing Intimacy.
    pub fn set_intimacy_level(&mut self, id: IntimacyId, level: IntimacyLevel) -> Result<&mut Self, CharacterMutationError> {
        let inner = self.intimacies.get_mut(&id).ok_or(CharacterMutationError::IntimacyError(IntimacyError::NotFound))?;

        inner.intimacy_level = level;
        Ok(self)
    }

    /// Sets the desciption for an existing Intimacy. 
    pub fn set_intimacy_description(&mut self, id: IntimacyId, description: &'source str) -> Result<&mut Self, CharacterMutationError> {
        let inner = self.intimacies.get_mut(&id).ok_or(CharacterMutationError::IntimacyError(IntimacyError::NotFound))?;

        inner.description = description;
        Ok(self)
    }
}