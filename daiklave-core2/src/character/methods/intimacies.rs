use std::collections::hash_map::Entry;

use crate::{
    intimacies::{
        intimacy::{IntimacyError, IntimacyMutation},
        Intimacies,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// The Intimacies a character holds an emotional connection to.
    pub fn intimacies(&'view self) -> Intimacies<'view, 'source> {
        Intimacies(self)
    }

    /// Adds a new Intimacy to a character.
    pub fn add_intimacy(
        &mut self,
        intimacy: &'source IntimacyMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        let IntimacyMutation { 
            intimacy_type, 
            level} = intimacy;

        if let Entry::Vacant(e) = self.intimacies.entry(intimacy_type.as_ref()) {
            e.insert(*level);
            Ok(self)
        } else {
            Err(CharacterMutationError::IntimacyError(
                IntimacyError::DuplicateIntimacy,
            ))
        }
    }

    /// Removes an Intimacy from the character.
    pub fn remove_intimacy(&mut self, intimacy: &'source IntimacyMutation) -> Result<&mut Self, CharacterMutationError> {
        if self.intimacies.remove(&intimacy.intimacy_type.as_ref()).is_some() {
            Ok(self)
        } else {
            Err(CharacterMutationError::IntimacyError(
                IntimacyError::NotFound,
            ))
        }
    }
}
