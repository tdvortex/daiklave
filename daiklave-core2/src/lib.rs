#![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

use abilities::{AbilityError, AbilityNameVanilla};
use armor::{
    armor_item::{artifact::ArtifactError, mundane::MundaneArmor, ArmorId, BaseArmorId},
    ArmorError,
};
use artifact::{Artifact, ArtifactId};
use attributes::{AttributeError, AttributeName};
use exaltation::exalt::{
    essence::{EssenceError, MoteCommitmentId, MotePoolName, OtherMoteCommitmentId},
    exalt_type::solar::NewSolar,
};
use health::{DamageLevel, WoundPenalty};
use hearthstones::{hearthstone::HearthstoneTemplate, HearthstoneError, HearthstoneId};
use martial_arts::{MartialArtsError, MartialArtsStyle, MartialArtsStyleId};
use merits::merit::{MeritId, StackableMeritId, StackableMerit, NonStackableMerit};
use name_and_concept::ConceptError;
use sorcery::SorceryError;
use thiserror::Error;

/// Structs related to a character's Abilities (skills) and specialties.
pub mod abilities;

/// Structs related to a character's Attributes.
pub mod attributes;

/// Structs related to a character's armor.
pub mod armor;

/// General properties of artifacts.
pub mod artifact;

/// Official page references.
pub mod book_reference;

/// Resources that are common across multiple types of Charms. Individual Charm
/// type definitions are recorded separately.
pub mod charms;

/// Traits which depend on being Mortal or Exalted.
pub mod exaltation;

/// A character builder with additional logic for bonus points, free starting
/// dots, and other constraints.
pub mod guided;

/// The Health struct and methods related to damage and healing.
pub mod health;

/// Hearthstones logic
pub mod hearthstones;

/// Martial Arts style logic
pub mod martial_arts;
/// Contains the Id enum and a variety of specific Id subtypes, to be used as
/// unique keys.
pub mod unique_id;

/// Sorcery logic
pub mod sorcery;

/// Logic for building and equipping weapons
pub mod weapons;

mod character;
mod character_memo;
pub(crate) mod craft;
mod merits;
mod name_and_concept;
mod willpower;

pub use character::Character;
pub use character_memo::CharacterMemo;
use weapons::{
    weapon::{equipped::EquipHand, mundane::MundaneWeapon, BaseWeaponId, Equipped, WeaponId},
    WeaponError,
};

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
    SetSolar(NewSolar),
    /// Spend motes, starting with one pool
    SpendMotes(MotePoolName, u8),
    /// Commit motes into a persistent effect, starting with one pool
    CommitMotes(OtherMoteCommitmentId, String, MotePoolName, u8),
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
    /// Adds a mundane weapon to the character.
    AddMundaneWeapon(BaseWeaponId, MundaneWeapon),
    /// Removes a mundane weapon from the character.
    RemoveMundaneWeapon(BaseWeaponId),
    /// Equips the specific weapon. For a OneHanded weapon, will equip into
    /// the specified hand, otherwise the parameter is ignored.
    EquipWeapon(WeaponId, Option<EquipHand>),
    /// Unequips the specific weapon at the specified equipped position.
    UnequipWeapon(WeaponId, Equipped),
    /// Add an artifact to the character, which may be a weapon, armor item,
    /// warstrider, or wonder.
    AddArtifact(Artifact),
    /// Removes an artifact from the character.
    RemoveArtifact(ArtifactId),
    /// Adds a piece of mundane armor.
    AddMundaneArmor(BaseArmorId, MundaneArmor),
    /// Removes a piece of mundane armor from the character.
    RemoveMundaneArmor(BaseArmorId),
    /// Equip a specific piece of armor.
    EquipArmor(ArmorId),
    /// Unequip any armor currently worn.
    UnequipArmor,
    /// Add a manse, its associated demense, and its associated hearthstone
    /// to the character.
    AddManse(String, String, HearthstoneId, HearthstoneTemplate),
    /// Add a hearthstone to a character without a manse.
    AddHearthstone(HearthstoneId, HearthstoneTemplate),
    /// Slot a hearthstone into an artifact.
    SlotHearthstone(ArtifactId, HearthstoneId),
    /// Unslot a hearthstone from its current position.
    UnslotHearthstone(HearthstoneId),
    /// Remove a hearthstone from the character, unslotting it in the process
    /// if needed.
    RemoveHearthstone(HearthstoneId),
    /// Attune to an artifact, committing motes to its ongoing use.
    AttuneArtifact(ArtifactId, MotePoolName),
    /// Add a stackable merit with an id for this instance and detail
    AddStackableMerit(StackableMeritId, StackableMerit),
    /// Add a nonstackable merit
    AddNonStackableMerit(NonStackableMerit),
    /// Remove a merit from the character
    RemoveMerit(MeritId),
}

/// An error representing something that could go wrong with a
/// CharacterMutation.
#[derive(Debug, Error)]
pub enum CharacterMutationError {
    /// Error related to abilities
    #[error("Abilities error")]
    AbilityError(#[from] AbilityError),
    /// Error related to armor
    #[error("Armor error")]
    ArmorError(#[from] ArmorError),
    /// Error related to artifacts
    #[error("Artifacts error")]
    ArtifactError(#[from] ArtifactError),
    /// Error related to Attributes
    #[error("Attribute Error")]
    AttributeError(#[from] AttributeError),
    /// Error occurring while trying to modify a character's concept
    #[error("Concept error")]
    ConceptError(#[from] ConceptError),
    /// Error related to Essence rating or mote pools
    #[error("Essence error")]
    EssenceError(#[from] EssenceError),
    /// Error related to hearthstones
    #[error("Hearthstone error")]
    HearthstoneError(#[from] HearthstoneError),
    /// Error related to Martial Arts
    #[error("Martial Arts error")]
    MartialArtsError(#[from] MartialArtsError),
    /// Error related to Sorcery
    #[error("Sorcery error")]
    SorceryError(#[from] SorceryError),
    /// Error related to weapons
    #[error("Weapons error")]
    WeaponError(#[from] WeaponError),
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
