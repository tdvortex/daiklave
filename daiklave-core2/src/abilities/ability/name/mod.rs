mod qualified;
mod vanilla;

pub use qualified::{AbilityNameQualified, AbilityNameQualifiedMutation};
pub use vanilla::AbilityNameVanilla;

use serde::{Deserialize, Serialize};

use crate::exaltation::exalt::exalt_type::solar::{
    caste::{
        dawn::{DawnCasteAbility, DawnSupernalAbility},
        eclipse::EclipseAbility,
        night::NightAbility,
        twilight::TwilightAbility,
        zenith::ZenithAbility,
    },
    charm::SolarCharmAbility,
};

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
    /// Bureaucracy
    Bureaucracy,
    /// Craft
    Craft,
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

impl From<EclipseAbility> for AbilityName {
    fn from(value: EclipseAbility) -> Self {
        match value {
            EclipseAbility::Bureaucracy => Self::Bureaucracy,
            EclipseAbility::Larceny => Self::Larceny,
            EclipseAbility::Linguistics => Self::Linguistics,
            EclipseAbility::Occult => Self::Occult,
            EclipseAbility::Presence => Self::Presence,
            EclipseAbility::Ride => Self::Ride,
            EclipseAbility::Sail => Self::Sail,
            EclipseAbility::Socialize => Self::Socialize,
        }
    }
}

impl From<NightAbility> for AbilityName {
    fn from(value: NightAbility) -> Self {
        match value {
            NightAbility::Athletics => Self::Athletics,
            NightAbility::Awareness => Self::Awareness,
            NightAbility::Dodge => Self::Dodge,
            NightAbility::Investigation => Self::Investigation,
            NightAbility::Larceny => Self::Larceny,
            NightAbility::Ride => Self::Ride,
            NightAbility::Stealth => Self::Stealth,
            NightAbility::Socialize => Self::Socialize,
        }
    }
}

impl From<TwilightAbility> for AbilityName {
    fn from(value: TwilightAbility) -> Self {
        match value {
            TwilightAbility::Bureaucracy => Self::Bureaucracy,
            TwilightAbility::Craft => Self::Craft,
            TwilightAbility::Integrity => Self::Integrity,
            TwilightAbility::Investigation => Self::Investigation,
            TwilightAbility::Linguistics => Self::Linguistics,
            TwilightAbility::Lore => Self::Lore,
            TwilightAbility::Medicine => Self::Medicine,
            TwilightAbility::Occult => Self::Occult,
        }
    }
}

impl From<ZenithAbility> for AbilityName {
    fn from(value: ZenithAbility) -> Self {
        match value {
            ZenithAbility::Athletics => Self::Athletics,
            ZenithAbility::Integrity => Self::Integrity,
            ZenithAbility::Performance => Self::Performance,
            ZenithAbility::Lore => Self::Lore,
            ZenithAbility::Presence => Self::Presence,
            ZenithAbility::Resistance => Self::Resistance,
            ZenithAbility::Survival => Self::Survival,
            ZenithAbility::War => Self::War,
        }
    }
}

impl From<SolarCharmAbility> for AbilityName {
    fn from(ability: SolarCharmAbility) -> Self {
        match ability {
            SolarCharmAbility::Archery => Self::Archery,
            SolarCharmAbility::Athletics => Self::Athletics,
            SolarCharmAbility::Awareness => Self::Awareness,
            SolarCharmAbility::Brawl => Self::Brawl,
            SolarCharmAbility::Bureaucracy => Self::Bureaucracy,
            SolarCharmAbility::Craft => Self::Craft,
            SolarCharmAbility::Dodge => Self::Dodge,
            SolarCharmAbility::Integrity => Self::Integrity,
            SolarCharmAbility::Investigation => Self::Investigation,
            SolarCharmAbility::Larceny => Self::Larceny,
            SolarCharmAbility::Linguistics => Self::Linguistics,
            SolarCharmAbility::Lore => Self::Lore,
            SolarCharmAbility::Medicine => Self::Medicine,
            SolarCharmAbility::Melee => Self::Melee,
            SolarCharmAbility::Occult => Self::Occult,
            SolarCharmAbility::Performance => Self::Performance,
            SolarCharmAbility::Presence => Self::Presence,
            SolarCharmAbility::Resistance => Self::Resistance,
            SolarCharmAbility::Ride => Self::Ride,
            SolarCharmAbility::Sail => Self::Sail,
            SolarCharmAbility::Socialize => Self::Socialize,
            SolarCharmAbility::Stealth => Self::Stealth,
            SolarCharmAbility::Survival => Self::Survival,
            SolarCharmAbility::Thrown => Self::Thrown,
            SolarCharmAbility::War => Self::War,
        }
    }
}
