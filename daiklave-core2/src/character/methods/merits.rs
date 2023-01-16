use std::collections::hash_map::Entry;

use crate::{
    merits::{
        merit::{
            MeritError, NonStackableMerit, NonStackableMeritId, StackableMerit, StackableMeritId,
        },
        Merits,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Access all Merits owned by the character.
    pub fn merits(&'view self) -> Merits<'view, 'source> {
        Merits(self)
    }

    /// Checks if a stackable merit can be added to the character
    pub fn check_add_stackable_merit(
        &self,
        stackable_merit_id: StackableMeritId,
        _stackable_merit: &'source StackableMerit,
    ) -> Result<(), CharacterMutationError> {
        if self.stackable_merits.contains_key(&stackable_merit_id) {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        } else {
            Ok(())
        }
    }

    /// Adds a stackable merit to the character.
    pub fn add_stackable_merit(
        &mut self,
        stackable_merit_id: StackableMeritId,
        stackable_merit: &'source StackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.stackable_merits.entry(stackable_merit_id) {
            e.insert(stackable_merit.as_ref());
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        }
    }

    /// Checks if a nonstackable merit can be added to the character.
    pub fn check_add_nonstackable_merit(
        &self,
        nonstackable_merit_id: NonStackableMeritId,
        _nonstackable_merit: &'source NonStackableMerit,
    ) -> Result<(), CharacterMutationError> {
        if self
            .nonstackable_merits
            .contains_key(&nonstackable_merit_id)
        {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        } else {
            Ok(())
        }
    }

    /// Adds a nonstackable merit to the character.
    pub fn add_nonstackable_merit(
        &mut self,
        nonstackable_merit_id: NonStackableMeritId,
        nonstackable_merit: &'source NonStackableMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.nonstackable_merits.entry(nonstackable_merit_id) {
            e.insert(nonstackable_merit.as_ref());
            Ok(self)
        } else {
            Err(CharacterMutationError::MeritError(
                MeritError::DuplicateMerit,
            ))
        }
    }
}
