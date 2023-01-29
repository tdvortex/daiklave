use std::collections::hash_map::Entry;

use crate::{
    intimacies::{
        intimacy::{IntimacyError, IntimacyMemo, AddIntimacy},
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
        intimacy: &'source AddIntimacy,
    ) -> Result<&mut Self, CharacterMutationError> {
        let AddIntimacy {
            intimacy_type,
            level,
        } = intimacy;

        if let Entry::Vacant(e) = self.intimacies.entry(intimacy_type.into()) {
            e.insert(*level);
            Ok(self)
        } else {
            Err(CharacterMutationError::IntimacyError(
                IntimacyError::DuplicateIntimacy,
            ))
        }
    }

    /// Removes an Intimacy from the character.
    pub fn remove_intimacy(
        &mut self,
        intimacy: &'source IntimacyMemo,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .intimacies
            .remove(&intimacy.intimacy_type)
            .is_some()
        {
            Ok(self)
        } else {
            Err(CharacterMutationError::IntimacyError(
                IntimacyError::NotFound,
            ))
        }
    }
}
