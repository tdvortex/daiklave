#![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

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

/// Flaws, which can earn extra experience or add an interesting dimension to
/// a character.
pub mod flaws;

/// A character builder with additional logic for bonus points, free starting
/// dots, and other constraints.
pub mod guided;

/// The Health struct and methods related to damage and healing.
pub mod health;

/// Hearthstones logic
pub mod hearthstones;

/// Character Intimacy logic
pub mod intimacies;

/// Languages of the Realm and Threshold
pub mod languages;

/// Martial Arts style logic
pub mod martial_arts;

/// Merits logic
pub mod merits;
/// Contains the Id enum and a variety of specific Id subtypes, to be used as
/// unique keys.
pub mod unique_id;

/// Sorcery logic
pub mod sorcery;

/// Logic for building and equipping weapons
pub mod weapons;

mod character;
pub(crate) mod craft;
mod name_and_concept;
mod willpower;

pub use character::{
    Character, CharacterEventSource, CharacterMemo, CharacterMutation, CharacterMutationError,
};
