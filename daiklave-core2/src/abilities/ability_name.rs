use serde::{Deserialize, Serialize};

use crate::exalt_state::exalt::exalt_type::solar::dawn::{DawnCasteAbility, DawnSupernalAbility};

use super::AbilityNameVanilla;

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

impl From<AbilityNameVanilla> for AbilityName {
    fn from(value: AbilityNameVanilla) -> Self {
        match value {
            AbilityNameVanilla::Archery => Self::Archery,
            AbilityNameVanilla::Athletics => Self::Athletics,
            AbilityNameVanilla::Awareness => Self::Awareness,
            AbilityNameVanilla::Brawl => Self::Brawl,
            AbilityNameVanilla::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanilla::Dodge => Self::Dodge,
            AbilityNameVanilla::Integrity => Self::Integrity,
            AbilityNameVanilla::Investigation => Self::Investigation,
            AbilityNameVanilla::Larceny => Self::Larceny,
            AbilityNameVanilla::Linguistics => Self::Linguistics,
            AbilityNameVanilla::Lore => Self::Lore,
            AbilityNameVanilla::Medicine => Self::Medicine,
            AbilityNameVanilla::Melee => Self::Melee,
            AbilityNameVanilla::Occult => Self::Occult,
            AbilityNameVanilla::Performance => Self::Performance,
            AbilityNameVanilla::Presence => Self::Presence,
            AbilityNameVanilla::Resistance => Self::Resistance,
            AbilityNameVanilla::Ride => Self::Ride,
            AbilityNameVanilla::Sail => Self::Sail,
            AbilityNameVanilla::Socialize => Self::Socialize,
            AbilityNameVanilla::Stealth => Self::Stealth,
            AbilityNameVanilla::Survival => Self::Survival,
            AbilityNameVanilla::Thrown => Self::Thrown,
            AbilityNameVanilla::War => Self::War,
        }
    }
}

impl From<DawnCasteAbility> for AbilityName {
    fn from(value: DawnCasteAbility) -> Self {
        match value {
            DawnCasteAbility::Archery => Self::Archery,
            DawnCasteAbility::Awareness => Self::Awareness,
            DawnCasteAbility::Brawl => Self::Brawl,
            DawnCasteAbility::Dodge => Self::Dodge,
            DawnCasteAbility::Melee => Self::Melee,
            DawnCasteAbility::Resistance => Self::Resistance,
            DawnCasteAbility::Thrown => Self::Thrown,
            DawnCasteAbility::War => Self::War,
        }
    }
}

impl From<DawnSupernalAbility> for AbilityName {
    fn from(value: DawnSupernalAbility) -> Self {
        match value {
            DawnSupernalAbility::Archery => Self::Archery,
            DawnSupernalAbility::Awareness => Self::Awareness,
            DawnSupernalAbility::Brawl => Self::Brawl,
            DawnSupernalAbility::Dodge => Self::Dodge,
            DawnSupernalAbility::MartialArts => Self::MartialArts,
            DawnSupernalAbility::Melee => Self::Melee,
            DawnSupernalAbility::Resistance => Self::Resistance,
            DawnSupernalAbility::Thrown => Self::Thrown,
            DawnSupernalAbility::War => Self::War,
        }
    }
}
