mod character;
mod character_view;
mod error;

pub use character::Abilities;
pub(crate) use character::Ability;
pub(crate) use character_view::AbilityView;
pub use character_view::AbilitiesView;
pub use error::{AddSpecialtyError, RemoveSpecialtyError, SetAbilityError};
use serde::{Deserialize, Serialize};

/// This is used to identify all ability ratings that must exist for a
/// character. It excludes all Craft abilities and MartialArts styles.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum AbilityNameVanilla {
    /// Archery
    Archery,
    /// Athletics
    Athletics,
    /// Awareness
    Awareness,
    /// Brawl
    Brawl,
    /// Bureaucracy
    Bureaucracy,
    /// Dodge
    Dodge,
    /// Integrity
    Integrity,
    /// Investigation
    Investigation,
    /// Larceny
    Larceny,
    /// Linguistics
    Linguistics,
    /// Lore
    Lore,
    /// Medicine
    Medicine,
    /// Melee
    Melee,
    /// Occult
    Occult,
    /// Performance
    Performance,
    /// Presence
    Presence,
    /// Resistance
    Resistance,
    /// Ride
    Ride,
    /// Sail
    Sail,
    /// Socialize
    Socialize,
    /// Stealth
    Stealth,
    /// Survival
    Survival,
    /// Thrown
    Thrown,
    /// War
    War,
}

/// This is used to identify all abilities, treating all Craft abilities as
/// equivalent and all MartialArts abilities as equivalent.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AbilityName {
    /// Archery
    Archery,
    /// Athletics
    Athletics,
    /// Awareness
    Awareness,
    /// Brawl
    Brawl,
    /// Craft
    Craft,
    /// Bureaucracy
    Bureaucracy,
    /// Dodge
    Dodge,
    /// Integrity
    Integrity,
    /// Investigation
    Investigation,
    /// Larceny
    Larceny,
    /// Linguistics
    Linguistics,
    /// Lore
    Lore,
    /// Martial Arts
    MartialArts,
    /// Medicine
    Medicine,
    /// Melee
    Melee,
    /// Occult
    Occult,
    /// Performance
    Performance,
    /// Presence
    Presence,
    /// Resistance
    Resistance,
    /// Ride
    Ride,
    /// Sail
    Sail,
    /// Socialize
    Socialize,
    /// Stealth
    Stealth,
    /// Survival
    Survival,
    /// Thrown
    Thrown,
    /// War
    War,
}
