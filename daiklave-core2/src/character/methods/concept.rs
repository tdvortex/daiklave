use crate::{Character, CharacterMutationError, ConceptError};

impl<'source> Character<'source> {
    /// Returns the character's concept (if any).
    pub fn concept(&self) -> Option<&str> {
        self.concept
    }

    /// Sets the character to the given concept.
    pub fn set_concept(
        &mut self,
        concept: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.concept = Some(concept);
        Ok(self)
    }

    /// Removes the character's concept.
    pub fn remove_concept(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.concept.is_none() {
            return Err(CharacterMutationError::ConceptError(
                ConceptError::NoConcept,
            ));
        }

        self.concept = None;
        Ok(self)
    }
}
