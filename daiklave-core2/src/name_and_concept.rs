use thiserror::Error;

use crate::{Character, CharacterMutationError, CharacterView};

/// An error occurring while attempting to set or remove a character's concept.
#[derive(Debug, Error)]
pub enum RemoveConceptError {
    #[error("character does not have a concept")]
    NoConcept,
}

impl Character {
    /// Returns the character's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the character's concept (if any).
    pub fn concept(&self) -> Option<&str> {
        self.concept.as_deref()
    }

    /// Checks if the character's name can be changed.
    pub fn check_set_name(&self, _name: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Checks if the character's concept can be set.
    pub fn check_set_concept(&self, _concept: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Checks if the character's concept can be removed.
    pub fn check_remove_concept(&self) -> Result<(), CharacterMutationError> {
        if self.concept().is_none() {
            Err(CharacterMutationError::RemoveConceptError(
                RemoveConceptError::NoConcept,
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the character's name.
    pub fn set_name(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_name(name)?;
        self.name = name.to_owned();
        Ok(self)
    }

    /// Sets the character to the given concept.
    pub fn set_concept(&mut self, concept: &str) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_concept(concept)?;
        self.concept = Some(concept.to_owned());
        Ok(self)
    }

    /// Removes the character's concept.
    pub fn remove_concept(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_concept()?;
        self.concept = None;
        Ok(self)
    }
}

impl<'source> CharacterView<'source> {
    /// Returns the character's name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the character's concept (if any).
    pub fn concept(&self) -> Option<&str> {
        self.concept
    }

    /// Checks if the character's name can be changed.
    pub fn check_set_name(&self, _name: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Checks if the character's concept can be set.
    pub fn check_set_concept(&self, _concept: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Checks if the character's concept can be removed.
    pub fn check_remove_concept(&self) -> Result<(), CharacterMutationError> {
        if self.concept().is_none() {
            Err(CharacterMutationError::RemoveConceptError(
                RemoveConceptError::NoConcept,
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the character's name.
    pub fn set_name(&mut self, name: &'source str) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_name(name)?;
        self.name = name;
        Ok(self)
    }

    /// Sets the character to the given concept.
    pub fn set_concept(
        &mut self,
        concept: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_concept(concept)?;
        self.concept = Some(concept);
        Ok(self)
    }

    /// Removes the character's concept.
    pub fn remove_concept(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_concept()?;
        self.concept = None;
        Ok(self)
    }
}
