use crate::{CharacterMutationError, Character, name_and_concept::ConceptError};

impl<'source> Character<'source> {
    /// Returns the character's concept (if any).
    pub fn concept(&self) -> Option<&str> {
        self.concept
    }

    /// Checks if the character's concept can be set.
    pub fn check_set_concept(&self, _concept: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Checks if the character's concept can be removed.
    pub fn check_remove_concept(&self) -> Result<(), CharacterMutationError> {
        if self.concept().is_none() {
            Err(CharacterMutationError::ConceptError(
                ConceptError::NoConcept,
            ))
        } else {
            Ok(())
        }
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