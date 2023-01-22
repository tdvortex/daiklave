use crate::{charms::charm::evocation::{Evocation, EvocationId}, Character, CharacterMutationError};

impl<'source> Character<'source> {
    /// Adds an evocation to the character.
    pub fn add_evocation(&mut self, evocation_id: EvocationId, evocation: &'source Evocation) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Removes an evocation from the character.
    pub fn remove_evocation(&mut self, evocation_id: EvocationId) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}