#![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

use id::{CharacterId, Id, SetIdError};
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Contains the Id enum and a variety of specific Id subtypes, to be used as
/// unique keys.
pub mod id;

/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    id: CharacterId,
}

/// A borrowed instance of a Character which references a CharacterEventSource
/// object, using &str instead of String. 
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterView {
    id: CharacterId,
}

/// The API for the character, expressed as an owned struct. Each mutation has
/// an associated pub method on Character and CharacterEventSource which 
/// returns Result<&mut Self, CharacterMutationError>. All API events also has
///  a "check_" variant which returns Result<(), CharacterMutationError>. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterMutation {
    /// Set the Character's Id.
    SetId(CharacterId),
}

impl Character {
    /// Returns the character's id.
    pub fn id(&self) -> CharacterId {
        self.id
    }

    /// Checks if a specific CharacterMutation can be safely applied. 
    pub fn check_mutation(&self, mutation: &CharacterMutation) -> Result<(), CharacterMutationError> {
        match mutation {
            CharacterMutation::SetId(id) => self.check_set_id(*id),
        }
    }

    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(&mut self, mutation: &CharacterMutation) -> Result<&mut Self, CharacterMutationError> {
        self.check_mutation(mutation)?;
        match mutation {
            CharacterMutation::SetId(id) => self.set_id(*id),
        }
    }

    /// Checks if the character's Id can be set.
    pub fn check_set_id(&self, id: CharacterId) -> Result<(), CharacterMutationError> {
        if let Id::Database(_) = *self.id {
            return Err(CharacterMutationError::SetIdError(SetIdError::DatabaseIdExists))
        } else if let Id::NonUnique = *id {
            return Err(CharacterMutationError::SetIdError(SetIdError::NonUniqueId))
        } else {
            Ok(())
        }
    }

    /// Sets the character's Id.
    pub fn set_id(&mut self, id: CharacterId) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_id(id)?;
        self.id = id;
        Ok(self)
    }
}

impl CharacterView {
    /// Returns the character's id.
    pub fn id(&self) -> CharacterId {
        self.id
    }

    /// Checks if a specific CharacterMutation can be safely applied. 
    pub fn check_mutation(&self, mutation: &CharacterMutation) -> Result<(), CharacterMutationError> {
        match mutation {
            CharacterMutation::SetId(id) => self.check_set_id(*id),
        }
    }

    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(&mut self, mutation: &CharacterMutation) -> Result<&mut Self, CharacterMutationError> {
        self.check_mutation(mutation)?;
        match mutation {
            CharacterMutation::SetId(id) => self.set_id(*id),
        }
    }

    /// Checks if the character's Id can be set.
    pub fn check_set_id(&self, id: CharacterId) -> Result<(), CharacterMutationError> {
        if let Id::Database(_) = *self.id {
            return Err(CharacterMutationError::SetIdError(SetIdError::DatabaseIdExists))
        } else if let Id::NonUnique = *id {
            return Err(CharacterMutationError::SetIdError(SetIdError::NonUniqueId))
        } else {
            Ok(())
        }
    }

    /// Sets the character's Id.
    pub fn set_id(&mut self, id: CharacterId) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_id(id)?;
        self.id = id;
        Ok(self)
    }
}

/// An error representing something that could go wrong with a 
/// CharacterMutation.
#[derive(Debug, Error)]
pub enum CharacterMutationError {
    /// Error occurring while trying to set CharacterId
    #[error("Cannot set character Id")]
    SetIdError(#[from] SetIdError),
}

/// A container to hold a successfully applied sequence of mutations, with
/// capability to undo/redo mutations. 
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CharacterEventSource{
    /// Previously applied mutations.
    history: Vec<CharacterMutation>,
    /// Mutations which were applied and then undone.
    future: Vec<CharacterMutation>,
}

impl CharacterEventSource {
    /// Constructs an owned Character from the event source history. Returns the
    /// default character if no events in the history.
    pub fn as_character(&self) -> Result<Character, CharacterMutationError> {
        self.history.iter().fold(Ok(Character::default()), |res, mutation| {
            res.and_then(|mut character| {character.apply_mutation(mutation)?; Ok(character)})
        })
    }

    /// Constructs a borrowed CharacterView from the event source history. 
    /// Returns the default character if no events in the history.
    pub fn as_character_view(&self) -> Result<CharacterView, CharacterMutationError> {
        self.history.iter().fold(Ok(CharacterView::default()), |res, mutation| {
            res.and_then(|mut view| {view.apply_mutation(mutation)?; Ok(view)})
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
    pub fn apply_mutation(&mut self, mutation: CharacterMutation) -> Result<&mut Self, CharacterMutationError> {
        self.as_character_view()?.check_mutation(&mutation)?;
        self.apply_mutation_unchecked(mutation);
        Ok(self)
    }
}