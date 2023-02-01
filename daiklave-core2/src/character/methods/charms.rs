use crate::{charms::{Charms, charm::{AddCharm, CharmName}}, Character, CharacterMutationError};

impl<'view, 'source> Character<'source> {
    /// Read the Charms (and Evocations and Spells) owned by the character.
    pub fn charms(&'view self) -> Charms<'view, 'source> {
        Charms(self)
    }

    pub fn add_charm(&mut self, add_charm: &'source AddCharm) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    pub fn remove_charm(&mut self, remove_charm: CharmName<'source>) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}
