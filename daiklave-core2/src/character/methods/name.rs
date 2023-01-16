use crate::{Character, CharacterMutationError};

impl<'source> Character<'source> {
    /// Returns the character's name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Checks if the character's name can be changed.
    pub fn check_set_name(&self, _name: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Sets the character's name.
    pub fn set_name(&mut self, name: &'source str) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_name(name)?;
        self.name = name;
        Ok(self)
    }
}
