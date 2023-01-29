use crate::{Character, CharacterMutation, CharacterMutationError};

use super::CharacterEvent;

/// A container to hold a successfully applied sequence of mutations, with
/// capability to undo/redo mutations.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CharacterEventSource {
    /// Previously applied mutations.
    history: Vec<CharacterMutation>,
    /// Mutations which were applied and then undone.
    future: Vec<CharacterMutation>,
}

impl<'source> CharacterEventSource {
    /// Constructs a borrowed Character from the event source history.
    /// Returns the default character if no events in the history.
    pub fn as_character(&'source self) -> Result<Character<'source>, CharacterMutationError> {
        self.history
            .iter()
            .fold(Ok(Character::default()), |res, mutation| {
                res.and_then(|mut character| {
                    character.apply_mutation(mutation)?;
                    Ok(character)
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

    /// Returns a Character with the last-applied operation undone. If there is
    /// no last operation, returns a default Character.
    pub fn undo(&'source mut self) -> Result<Character<'source>, CharacterMutationError> {
        if let Some(mutation) = self.history.pop() {
            self.future.push(mutation);
        }
        self.as_character()
    }

    /// Returns a Character with the last-undone operation redone.
    /// Unlike apply_mutation, this does not clear the redoable history.
    pub fn redo(&'source mut self) -> Result<Character<'source>, CharacterMutationError> {
        if let Some(mutation) = self.future.pop() {
            self.history.push(mutation);
        }
        self.as_character()
    }

    /// Applies a character mutation, and returns the Character after the
    pub fn apply_mutation(
        &'source mut self,
        mutation: impl Into<CharacterMutation>,
    ) -> Result<Character<'source>, CharacterMutationError> {
        let mutation: CharacterMutation = mutation.into();
        self.as_character()?.apply_mutation(&mutation)?;
        self.future = Vec::new();
        self.history.push(mutation);
        self.as_character()
    }

    /// Applies an event to the event source and returns the Character after 
    /// the event.
    pub fn apply_event(
        &'source mut self,
        event: impl CharacterEvent<'source>
    ) -> Result<Character<'source>, CharacterMutationError> {
        event.apply_event(self)
    }
}
