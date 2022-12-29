#![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

use abilities::{
    Abilities, AbilitiesView, AddSpecialtyError, RemoveSpecialtyError, SetAbilityError,
};
use attributes::SetAttributesError;
use essence::{CommitMotesError, SpendMotesError};
use essence::{RecoverMotesError, SetEssenceRatingError, UncommitMotesError};
use exalt_type::{ExaltState, ExaltStateView};
use id::{CharacterId, SetIdError};
use name_and_concept::RemoveConceptError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Contains the Id enum and a variety of specific Id subtypes, to be used as
/// unique keys.
pub mod id;

/// Traits which are unique to being a Solar Exalted.
pub use exalt_type::SolarTraits;

/// A character builder with additional logic for bonus points, free starting
/// dots, and other constraints.
pub mod guided;

pub use abilities::{AbilityName, AbilityNameVanilla};
pub use attributes::{AttributeName, Attributes};
pub use essence::CommittedMotesId;
pub use essence::MotePool;
use willpower::Willpower;

mod abilities;
mod attributes;
mod essence;
mod exalt_type;
mod health;
mod name_and_concept;
mod willpower;

pub use health::{DamageLevel, Health, WoundPenalty};
/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    id: CharacterId,
    name: String,
    concept: Option<String>,
    exalt_state: ExaltState,
    willpower: Willpower,
    health: Health,
    attributes: Attributes,
    abilities: Abilities,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: "New Character".to_owned(),
            concept: Default::default(),
            exalt_state: Default::default(),
            willpower: Default::default(),
            health: Default::default(),
            attributes: Default::default(),
            abilities: Default::default(),
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
    exalt_state: ExaltStateView<'source>,
    willpower: Willpower,
    health: Health,
    attributes: Attributes,
    abilities: AbilitiesView<'source>,
}

impl<'source> Default for CharacterView<'source> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: "New Character",
            concept: Default::default(),
            exalt_state: Default::default(),
            willpower: Default::default(),
            health: Default::default(),
            attributes: Default::default(),
            abilities: Default::default(),
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
    /// Set character to be mortal
    SetMortal,
    /// Set character to be Solar
    SetSolar(SolarTraits),
    /// Spend motes, starting with one pool
    SpendMotes(MotePool, u8),
    /// Commit motes into a persistent effect, starting with one pool
    CommitMotes(CommittedMotesId, String, MotePool, u8),
    /// Recover motes, always starting from peripheral
    RecoverMotes(u8),
    /// Uncommit motes from a peristent effect
    UncommitMotes(CommittedMotesId),
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
            CharacterMutation::SetMortal => self.check_set_mortal(),
            CharacterMutation::SetSolar(solar_traits) => self.check_set_solar(solar_traits),
            CharacterMutation::SpendMotes(first, amount) => self.check_spend_motes(*first, *amount),
            CharacterMutation::CommitMotes(id, name, first, amount) => {
                self.check_commit_motes(id, name, *first, *amount)
            }
            CharacterMutation::RecoverMotes(amount) => self.check_recover_motes(*amount),
            CharacterMutation::UncommitMotes(id) => self.check_uncommit_motes(id),
            CharacterMutation::SetEssenceRating(rating) => self.check_set_essence_rating(*rating),
            CharacterMutation::SetCurrentWillpower(amount) => {
                self.check_set_current_willpower(*amount)
            }
            CharacterMutation::SetWillpowerRating(dots) => self.check_set_willpower_rating(*dots),
            CharacterMutation::SetWoundPenalties(wound_penalties) => {
                self.check_set_wound_penalties(wound_penalties)
            }
            CharacterMutation::TakeDamage(damage_level, amount) => {
                self.check_take_damage(*damage_level, *amount)
            }
            CharacterMutation::HealDamage(amount) => self.check_heal_damage(*amount),
            CharacterMutation::SetAttribute(attribute_name, dots) => {
                self.check_set_attribute(*attribute_name, *dots)
            }
            CharacterMutation::SetAbilityDots(ability_name, dots) => {
                self.check_set_ability_dots(*ability_name, *dots)
            }
            CharacterMutation::AddSpecialty(ability_name, specialty) => {
                self.check_add_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::RemoveSpecialty(ability_name, specialty) => {
                self.check_remove_specialty(*ability_name, specialty.as_str())
            }
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
            CharacterMutation::SetMortal => self.set_mortal(),
            CharacterMutation::SetSolar(solar_traits) => self.set_solar(solar_traits),
            CharacterMutation::SpendMotes(first, amount) => self.spend_motes(*first, *amount),
            CharacterMutation::CommitMotes(id, name, first, amount) => {
                self.commit_motes(id, name, *first, *amount)
            }
            CharacterMutation::RecoverMotes(amount) => self.recover_motes(*amount),
            CharacterMutation::UncommitMotes(id) => self.uncommit_motes(id),
            CharacterMutation::SetEssenceRating(rating) => self.set_essence_rating(*rating),
            CharacterMutation::SetCurrentWillpower(amount) => self.set_current_willpower(*amount),
            CharacterMutation::SetWillpowerRating(dots) => self.set_willpower_rating(*dots),
            CharacterMutation::SetWoundPenalties(wound_penalties) => {
                self.set_wound_penalties(wound_penalties)
            }
            CharacterMutation::TakeDamage(damage_level, amount) => {
                self.take_damage(*damage_level, *amount)
            }
            CharacterMutation::HealDamage(amount) => self.heal_damage(*amount),
            CharacterMutation::SetAttribute(attribute_name, dots) => {
                self.set_attribute(*attribute_name, *dots)
            }
            CharacterMutation::SetAbilityDots(ability_name, dots) => {
                self.set_ability_dots(*ability_name, *dots)
            }
            CharacterMutation::AddSpecialty(ability_name, specialty) => {
                self.add_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::RemoveSpecialty(ability_name, specialty) => {
                self.remove_specialty(*ability_name, specialty.as_str())
            }
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
            CharacterMutation::SetMortal => self.check_set_mortal(),
            CharacterMutation::SetSolar(solar_traits) => self.check_set_solar(solar_traits),
            CharacterMutation::SpendMotes(first, amount) => self.check_spend_motes(*first, *amount),
            CharacterMutation::CommitMotes(id, name, first, amount) => {
                self.check_commit_motes(id, name, *first, *amount)
            }
            CharacterMutation::RecoverMotes(amount) => self.check_recover_motes(*amount),
            CharacterMutation::UncommitMotes(id) => self.check_uncommit_motes(id),
            CharacterMutation::SetEssenceRating(rating) => self.check_set_essence_rating(*rating),
            CharacterMutation::SetCurrentWillpower(amount) => {
                self.check_set_current_willpower(*amount)
            }
            CharacterMutation::SetWillpowerRating(dots) => self.check_set_willpower_rating(*dots),
            CharacterMutation::SetWoundPenalties(wound_penalties) => {
                self.check_set_wound_penalties(wound_penalties)
            }
            CharacterMutation::TakeDamage(damage_level, amount) => {
                self.check_take_damage(*damage_level, *amount)
            }
            CharacterMutation::HealDamage(amount) => self.check_heal_damage(*amount),
            CharacterMutation::SetAttribute(attribute_name, dots) => {
                self.check_set_attribute(*attribute_name, *dots)
            }
            CharacterMutation::SetAbilityDots(ability_name, dots) => {
                self.check_set_ability_dots(*ability_name, *dots)
            }
            CharacterMutation::AddSpecialty(ability_name, specialty) => {
                self.check_add_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::RemoveSpecialty(ability_name, specialty) => {
                self.check_remove_specialty(*ability_name, specialty.as_str())
            }
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
            CharacterMutation::SetMortal => self.set_mortal(),
            CharacterMutation::SetSolar(solar_traits) => self.set_solar(solar_traits),
            CharacterMutation::SpendMotes(first, amount) => self.spend_motes(*first, *amount),
            CharacterMutation::CommitMotes(id, name, first, amount) => {
                self.commit_motes(id, name, *first, *amount)
            }
            CharacterMutation::RecoverMotes(amount) => self.recover_motes(*amount),
            CharacterMutation::UncommitMotes(id) => self.uncommit_motes(id),
            CharacterMutation::SetEssenceRating(rating) => self.set_essence_rating(*rating),
            CharacterMutation::SetCurrentWillpower(amount) => self.set_current_willpower(*amount),
            CharacterMutation::SetWillpowerRating(dots) => self.set_willpower_rating(*dots),
            CharacterMutation::SetWoundPenalties(wound_penalties) => {
                self.set_wound_penalties(wound_penalties)
            }
            CharacterMutation::TakeDamage(damage_level, amount) => {
                self.take_damage(*damage_level, *amount)
            }
            CharacterMutation::HealDamage(amount) => self.heal_damage(*amount),
            CharacterMutation::SetAttribute(attribute_name, dots) => {
                self.set_attribute(*attribute_name, *dots)
            }
            CharacterMutation::SetAbilityDots(ability_name, dots) => {
                self.set_ability_dots(*ability_name, *dots)
            }
            CharacterMutation::AddSpecialty(ability_name, specialty) => {
                self.add_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::RemoveSpecialty(ability_name, specialty) => {
                self.remove_specialty(*ability_name, specialty.as_str())
            }
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
    /// Error occurring while trying to spend motes
    #[error("Cannot spend motes")]
    SpendMotesError(#[from] SpendMotesError),
    /// Error occurring while trying to commit motes
    #[error("Cannot commit motes")]
    CommitMotesError(#[from] CommitMotesError),
    /// Error occurring while trying to recover motes
    #[error("Cannot recover motes")]
    RecoverMotesError(#[from] RecoverMotesError),
    /// Error occuring while trying to uncommit motes
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
