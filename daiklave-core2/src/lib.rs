#![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

use id::{CharacterId, SetIdError};
use name_and_concept::RemoveConceptError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Contains the Id enum and a variety of specific Id subtypes, to be used as
/// unique keys.
pub mod id;

mod name_and_concept;
/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    id: CharacterId,
    name: String,
    concept: Option<String>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: "New Character".to_owned(),
            concept: Default::default(),
        }
    }
}

/// A borrowed instance of a Character which references a CharacterEventSource
/// object, using &str instead of String.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterView<'source> {
    id: CharacterId,
    name: &'source str,
    concept: Option<&'source str>,
}

impl<'source> Default for CharacterView<'source> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: "New Character",
            concept: Default::default(),
        }
    }
}

/// The API for the character, expressed as an owned struct. Each mutation has
/// an associated pub method on Character and CharacterEventSource which
/// returns Result<&mut Self, CharacterMutationError>. All API events also has
///  a "check_" variant which returns Result<(), CharacterMutationError>.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterMutation {
    /// Set the Character's Id.
    SetId(CharacterId),
    /// Set the Character's name
    SetName(String),
    /// Set the Character's concept
    SetConcept(String),
    /// Remove the Character's concept
    RemoveConcept,
}

impl Character {
    /// Checks if a specific CharacterMutation can be safely applied.
    pub fn check_mutation(
        &self,
        mutation: &CharacterMutation,
    ) -> Result<(), CharacterMutationError> {
        match mutation {
            CharacterMutation::SetId(id) => self.check_set_id(*id),
            CharacterMutation::SetName(name) => self.check_set_name(name.as_str()),
            CharacterMutation::SetConcept(concept) => self.check_set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.check_remove_concept(),
        }
    }

    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(
        &mut self,
        mutation: &CharacterMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_mutation(mutation)?;
        match mutation {
            CharacterMutation::SetId(id) => self.set_id(*id),
            CharacterMutation::SetName(name) => self.set_name(name.as_str()),
            CharacterMutation::SetConcept(concept) => self.set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.remove_concept(),
        }
    }
}

impl<'source> CharacterView<'source> {
    /// Checks if a specific CharacterMutation can be safely applied.
    pub fn check_mutation(
        &self,
        mutation: &CharacterMutation,
    ) -> Result<(), CharacterMutationError> {
        match mutation {
            CharacterMutation::SetId(id) => self.check_set_id(*id),
            CharacterMutation::SetName(name) => self.check_set_name(name.as_str()),
            CharacterMutation::SetConcept(concept) => self.check_set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.check_remove_concept(),
        }
    }

    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(
        &mut self,
        mutation: &'source CharacterMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_mutation(mutation)?;
        match mutation {
            CharacterMutation::SetId(id) => self.set_id(*id),
            CharacterMutation::SetName(name) => self.set_name(name.as_str()),
            CharacterMutation::SetConcept(concept) => self.set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.remove_concept(),
        }
    }
}

/// An error representing something that could go wrong with a
/// CharacterMutation.
#[derive(Debug, Error)]
pub enum CharacterMutationError {
    /// Error occurring while trying to set CharacterId
    #[error("Cannot set character Id")]
    SetIdError(#[from] SetIdError),
    /// Error occurring while trying to remove concept
    #[error("Cannot remove character concept")]
    RemoveConceptError(#[from] RemoveConceptError),
}

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
    /// Constructs an owned Character from the event source history. Returns the
    /// default character if no events in the history.
    pub fn as_character(&self) -> Result<Character, CharacterMutationError> {
        self.history
            .iter()
            .fold(Ok(Character::default()), |res, mutation| {
                res.and_then(|mut character| {
                    character.apply_mutation(mutation)?;
                    Ok(character)
                })
            })
    }

    /// Constructs a borrowed CharacterView from the event source history.
    /// Returns the default character if no events in the history.
    pub fn as_character_view(&self) -> Result<CharacterView, CharacterMutationError> {
        self.history
            .iter()
            .fold(Ok(CharacterView::default()), |res, mutation| {
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
