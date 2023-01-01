#![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

use abilities::{AbilityNameVanilla, AddSpecialtyError, RemoveSpecialtyError, SetAbilityError};
use attributes::{AttributeName, SetAttributesError};
use exalt_state::exalt::{
    essence::{
        CommitMotesError, MoteCommitmentId, MotePoolName, RecoverMotesError, SetEssenceRatingError,
        SpendMotesError, UncommitMotesError,
    },
    exalt_type::solar::Solar,
};
use health::{DamageLevel, WoundPenalty};
use martial_arts::{
    AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId, RemoveMartialArtsStyleError,
    SetMartialArtsDotsError,
};
use name_and_concept::RemoveConceptError;
use sorcery::SorceryError;
use thiserror::Error;

/// Structs related to a character's Abilities (skills) and specialties.
pub mod abilities;

/// Structs related to a character's Attributes.
pub mod attributes;

/// Official page references.
pub mod book_reference;

/// Resources that are common across multiple types of Charms. Individual Charm
/// type definitions are recorded separately.
pub mod charms;

/// Traits which depend on being Mortal or Exalted.
pub mod exalt_state;

/// A character builder with additional logic for bonus points, free starting
/// dots, and other constraints.
pub mod guided;

/// The Health struct and methods related to damage and healing.
pub mod health;

/// Contains the Id enum and a variety of specific Id subtypes, to be used as
/// unique keys.
pub mod id;
/// Martial Arts style logic
pub mod martial_arts;

/// Sorcery logic
pub mod sorcery;

/// Logic for building and equipping weapons
pub mod weapons;

mod armor;
mod character;
mod character_view;
pub(crate) mod craft;
mod name_and_concept;
mod willpower;

pub use character::Character;
pub use character_view::CharacterView;

/// The API for the character, expressed as an owned struct. Each mutation has
/// an associated pub method on Character and CharacterEventSource which
/// returns Result<&mut Self, CharacterMutationError>. All API events also has
///  a "check_" variant which returns Result<(), CharacterMutationError>.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterMutation {
    /// Set the Character's name
    SetName(String),
    /// Set the Character's concept
    SetConcept(String),
    /// Remove the Character's concept
    RemoveConcept,
    /// Set character to be mortal
    SetMortal,
    /// Set character to be Solar
    SetSolar(Box<Solar>),
    /// Spend motes, starting with one pool
    SpendMotes(MotePoolName, u8),
    /// Commit motes into a persistent effect, starting with one pool
    CommitMotes(MoteCommitmentId, String, MotePoolName, u8),
    /// Recover motes, always starting from peripheral
    RecoverMotes(u8),
    /// Uncommit motes from a peristent effect
    UncommitMotes(MoteCommitmentId),
    /// Set the Essence rating of the character. Note: also ends all mote
    /// commitments and recovers all motes.
    SetEssenceRating(u8),
    /// Sets the current willpower value of the character.
    SetCurrentWillpower(u8),
    /// Sets the permanent willpower rating of the character. Also resets
    /// current willpower to permanent rating.
    SetWillpowerRating(u8),
    /// Changes the character's health track to have the specified wound
    /// penalties.
    SetWoundPenalties(Vec<WoundPenalty>),
    /// Adds the specified amount and type of damage to the character's
    /// health track, accounting for overflows.
    TakeDamage(DamageLevel, u8),
    /// Heals the specified amount of damage, always bashing then lethal then
    /// aggravated.
    HealDamage(u8),
    /// Sets an attribute to a specific rating.
    SetAttribute(AttributeName, u8),
    /// Sets an ability (other than Craft or Martial Arts) to a dot rating.
    SetAbilityDots(AbilityNameVanilla, u8),
    /// Adds a specialty to a non-zero, non-Craft, non-Martial Arts ability.
    AddSpecialty(AbilityNameVanilla, String),
    /// Removes a specialty from a non-Craft, non-Martial Arts ability.
    RemoveSpecialty(AbilityNameVanilla, String),
    /// Adds a Martial Arts style to a character. This purchases the
    /// MartialArtist merit for the style, but does not grant any Martial Arts
    /// dots or Martial Arts charms.
    AddMartialArtsStyle(MartialArtsStyleId, MartialArtsStyle),
    /// Removes a Martial Arts style from a character, including the merit,
    /// associated ability dots, specialties, and Charms.
    RemoveMartialArtsStyle(MartialArtsStyleId),
    /// Sets the Ability dots for a specific Martial Arts style.
    SetMartialArtsDots(MartialArtsStyleId, u8),
    /// Sets the Craft dots for a particular focus area.
    SetCraftDots(String, u8),
}

/// An error representing something that could go wrong with a
/// CharacterMutation.
#[derive(Debug, Error)]
pub enum CharacterMutationError {
    /// Error occurring while trying to remove concept
    #[error("Cannot remove character concept")]
    RemoveConceptError(#[from] RemoveConceptError),
    /// Error occurring while trying to spend motes
    #[error("Cannot spend motes")]
    SpendMotesError(#[from] SpendMotesError),
    /// Error occurring while trying to commit motes
    #[error("Cannot commit motes")]
    CommitMotesError(#[from] CommitMotesError),
    /// Error occurring while trying to recover motes
    #[error("Cannot recover motes")]
    RecoverMotesError(#[from] RecoverMotesError),
    /// Error occurring while trying to uncommit motes
    #[error("Cannot uncommit motes")]
    UncommitMotesError(#[from] UncommitMotesError),
    /// Error occurring while trying to set essence rating
    #[error("Cannot set Essence rating")]
    SetEssenceRatingError(#[from] SetEssenceRatingError),
    /// Error occurring while trying to set an attribute rating
    #[error("Cannot set attribute rating")]
    SetAttributesError(#[from] SetAttributesError),
    /// Error occurring while trying to set an ability dot rating
    #[error("Cannot set ability rating")]
    SetAbilityError(#[from] SetAbilityError),
    /// Error occurring while trying to add a specialty
    #[error("Cannot add specialty")]
    AddSpecialtyError(#[from] AddSpecialtyError),
    /// Error occurring while trying to remove a specialty
    #[error("Cannot remove specialty")]
    RemoveSpecialtyError(#[from] RemoveSpecialtyError),
    /// Error occurring while trying to add a Martial Arts style
    #[error("Cannot add Martial Arts style")]
    AddMartialArtsStyleError(#[from] AddMartialArtsStyleError),
    /// Error occurring while trying to remove a Martial Arts style
    #[error("Cannot remove Martial Arts style")]
    RemoveMartialArtsStyleError(#[from] RemoveMartialArtsStyleError),
    /// Error occurring while trying to set Martial Arts dots
    #[error("Cannot set Martial Arts dots")]
    SetMartialArtsDotsError(#[from] SetMartialArtsDotsError),
    /// Error occurring while trying to add a Sorcery circle
    #[error("Cannot add Sorcery circle")]
    AddSorceryCircleError(#[from] SorceryError),
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
