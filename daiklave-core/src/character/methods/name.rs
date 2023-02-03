use crate::{Character, CharacterMutationError};

impl<'source> Character<'source> {
    /// Returns the character's name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Sets the character's name.
    pub fn set_name(&mut self, name: &'source str) -> Result<&mut Self, CharacterMutationError> {
        self.name = name;
        Ok(self)
    }
}
