#![allow(clippy::large_enum_variant)] // TODO: box stuff where needed
// #![warn(missing_docs)]
//! **Daiklave** is a Rust character sheet application, designed to be as
//! flexible as a paper sheet, as easy to use as a virtual tabletop (VTT),
//! with full Discord integration for over-the-internet play.

/// Defines the Interface for accessing character abilities (aka skills) and
/// specialties.
pub mod abilities;

/// Defines the AnimaLevel enum.
pub mod anima;

/// Defines constructors and types for individual armor pieces (ArmorItem),
/// as well as the interface to own and equip them.
pub mod armor;

pub mod artifact;

/// Defines the interface for character attributes (Strength, Intelligence,
/// etc.).
pub mod attributes;

mod brainstorming;

/// Defines the properties of a campaign, to which players and characters
/// may be added.
pub mod campaign;

/// Defines he core Character struct, and its builders. Also includes values
/// that are common to all player characters with simple interfaces, including
/// Willpower and Experience.
pub mod character;

pub mod charms;
pub mod craft;

/// Defines a DataSource field, allowing for resources (Merits, Charms,
/// Weapons, etc.) to be specified from either an official sourcebook
/// or with a custom creator Id.
pub mod data_source;

/// Defines the Essence trait and MotePool interface, which is shared by
/// most Exalt varieties.
pub mod essence;

/// Defines a trait to allow a character to switch between various Exalt
/// types (as well as Mortal and MortalSorcerer).
pub(crate) mod exalt_type;

/// Defines the Health interface for damage and healing.
pub mod health;

/// Defines an Id enum to uniquely identify a resource, both client-side
/// and database-side.
pub mod id;

pub mod initiative;

/// Defines the Intimacy item and its collection, Intimacies.
pub mod intimacies;

/// Defines the Limit track for Celestial Exalted (Solars and Lunars).
pub mod limit;

/// Defines the configuration related to Martial Arts, including
/// MartialArtsStyles, the MartialArts ability, and MartialArtsCharms.
pub mod martial_arts;

/// Defines the structure of an individual Merit and the Merits collection.
pub mod merits;

/// Defines a Player, who may own Characters and participate in Campaigns.
pub mod player;

/// TODO: delete this, fold important stuff into Merits
pub mod prerequisite;

/// Defines the Solar Exalt type, including its five Castes and its unique
/// systems (such as Supernal abilities and Eclipse charms).
pub mod solar;

/// Defines the Sorcery interface.
pub mod sorcery;

/// Defines the Weapon interface for creating weaponry (including daiklaves!)
/// and the character Weapons interface for adding, removing, and unequipping
/// them.
pub mod weapons;

pub use character::Character;
