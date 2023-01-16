use crate::{Character, CharacterMutation, CharacterMutationError};

/// A container to hold a successfully applied sequence of mutations, with
/// capability to undo/redo mutations.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CharacterEventSource {
    /// Previously applied mutations.
    history: Vec<CharacterMutation>,
    /// Mutations which were applied and then undone.
    future: Vec<CharacterMutation>,
}

impl CharacterEventSource {
    /// Constructs a borrowed CharacterView from the event source history.
    /// Returns the default character if no events in the history.
    pub fn as_character_view(&self) -> Result<Character, CharacterMutationError> {
        self.history
            .iter()
            .fold(Ok(Character::default()), |res, mutation| {
                res.and_then(|mut view| {
                    view.apply_mutation(mutation)?;
                    Ok(view)
                })
            })
    }

    /// Returns true if there is any mutation to undo.
    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    /// Returns true if there is any mutation to redo.
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Undoes the last mutation (if any), returns true if any undo occurred.
    pub fn undo(&mut self) -> bool {
        if let Some(mutation) = self.history.pop() {
            self.future.push(mutation);
            true
        } else {
            false
        }
    }

    /// Redoes the last undone mutation (if any), returns true if any redo
    /// occurred.
    pub fn redo(&mut self) -> bool {
        if let Some(mutation) = self.future.pop() {
            self.history.push(mutation);
            true
        } else {
            false
        }
    }

    /// Applies a character mutation without checking validity. If an invalid
    /// character mutation is passed, attempts to reconstruct using
    /// as_character or as_view may fail. This can be corrected by using undo
    /// to revert the invalid mutation.
    pub fn apply_mutation_unchecked(&mut self, mutation: CharacterMutation) {
        self.future = Vec::new();
        self.history.push(mutation);
    }

    /// Applies a character mutation. Returns CharacterMutationError if
    /// unsuccessful with no other changes. Erases redo-able mutations if
    /// successful.
    pub fn apply_mutation(
        &mut self,
        mutation: CharacterMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.as_character_view()?.check_mutation(&mutation)?;
        self.apply_mutation_unchecked(mutation);
        Ok(self)
    }
}
