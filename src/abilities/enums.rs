use eyre::eyre;
use serde::{Deserialize, Serialize};

// The name of an ability, excluding any Craft focus areas or Martial Arts styles.
/// This is useful for most Craft Charms and nonspecific combat merits like Quick Draw.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AbilityNameNoSubskill {
    /// The Archery ability
    Archery,
    /// The Athletics ability
    Athletics,
    /// The Awareness ability
    Awareness,
    /// The Brawl ability
    Brawl,
    /// The Bureaucracy ability
    Bureaucracy,
    /// The Craft ability, irrespective of focus area
    Craft,
    /// The Dodge ability
    Dodge,
    /// The Integrity ability
    Integrity,
    /// The Investigation ability
    Investigation,
    /// The Larceny ability
    Larceny,
    /// The Linguistics ability
    Linguistics,
    /// The Lore ability
    Lore,
    /// The MartialArts ability, irrespective of style
    MartialArts,
    /// The Medicine ability
    Medicine,
    /// The Melee ability
    Melee,
    /// The Occult ability
    Occult,
    /// The Performance ability
    Performance,
    /// The Presence ability
    Presence,
    /// The Resistance ability
    Resistance,
    /// The Ride ability
    Ride,
    /// The Sail ability
    Sail,
    /// The Socialize ability
    Socialize,
    /// The Stealth ability
    Stealth,
    /// The Survival ability
    Survival,
    /// The Thrown ability
    Thrown,
    /// The War ability
    War,
}

/// The name of an Ability, including a specific Craft focus area or Martial Arts style if appropriate.
/// This is useful for querying a specific ability's dots (e.g. Craft(Masonry) vs Craft(Basketweaving))
/// or its specialties.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AbilityName<'a> {
    /// The Archery ability
    Archery,
    /// The Athletics ability
    Athletics,
    /// The Awareness ability
    Awareness,
    /// The Brawl ability
    Brawl,
    /// The Bureaucracy ability
    Bureaucracy,
    /// The Craft ability, augmented with a specific focus area (such as Craft("Artifacts"))
    Craft(&'a str),
    /// The Dodge ability
    Dodge,
    /// The Integrity ability
    Integrity,
    /// The Investigation ability
    Investigation,
    /// The Larceny ability
    Larceny,
    /// The Linguistics ability
    Linguistics,
    /// The Lore ability
    Lore,
    /// The MartialArts ability, augmented with a specific style (such as MartialArts("Crane Style"))
    MartialArts(&'a str),
    /// The Medicine ability
    Medicine,
    /// The Melee ability
    Melee,
    /// The Occult ability
    Occult,
    /// The Performance ability
    Performance,
    /// The Presence ability
    Presence,
    /// The Resistance ability
    Resistance,
    /// The Ride ability
    Ride,
    /// The Sail ability
    Sail,
    /// The Socialize ability
    Socialize,
    /// The Stealth ability
    Stealth,
    /// The Survival ability
    Survival,
    /// The Thrown ability
    Thrown,
    /// The War ability
    War,
}

/// This is used to identify all ability ratings that must exist for a 
/// character. This means it specifically excludes all Craft abilities and
/// MartialArts styles.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AbilityNameVanilla {
    /// The Archery ability
    Archery,
    /// The Athletics ability
    Athletics,
    /// The Awareness ability
    Awareness,
    /// The Brawl ability
    Brawl,
    /// The Bureaucracy ability
    Bureaucracy,
    /// The Dodge ability
    Dodge,
    /// The Integrity ability
    Integrity,
    /// The Investigation ability
    Investigation,
    /// The Larceny ability
    Larceny,
    /// The Linguistics ability
    Linguistics,
    /// The Lore ability
    Lore,
    /// The Medicine ability
    Medicine,
    /// The Melee ability
    Melee,
    /// The Occult ability
    Occult,
    /// The Performance ability
    Performance,
    /// The Presence ability
    Presence,
    /// The Resistance ability
    Resistance,
    /// The Ride ability
    Ride,
    /// The Sail ability
    Sail,
    /// The Socialize ability
    Socialize,
    /// The Stealth ability
    Stealth,
    /// The Survival ability
    Survival,
    /// The Thrown ability
    Thrown,
    /// The War ability
    War,
}

impl From<AbilityName<'_>> for AbilityNameNoSubskill {
    fn from(value: AbilityName) -> Self {
        match value {
            AbilityName::Archery => Self::Archery,
            AbilityName::Athletics => Self::Athletics,
            AbilityName::Awareness => Self::Awareness,
            AbilityName::Brawl => Self::Brawl,
            AbilityName::Bureaucracy => Self::Bureaucracy,
            AbilityName::Craft(_) => Self::Craft,
            AbilityName::Dodge => Self::Dodge,
            AbilityName::Integrity => Self::Integrity,
            AbilityName::Investigation => Self::Investigation,
            AbilityName::Larceny => Self::Larceny,
            AbilityName::Linguistics => Self::Linguistics,
            AbilityName::Lore => Self::Lore,
            AbilityName::MartialArts(_) => Self::MartialArts,
            AbilityName::Medicine => Self::Medicine,
            AbilityName::Melee => Self::Melee,
            AbilityName::Occult => Self::Occult,
            AbilityName::Performance => Self::Performance,
            AbilityName::Presence => Self::Presence,
            AbilityName::Resistance => Self::Resistance,
            AbilityName::Ride => Self::Ride,
            AbilityName::Sail => Self::Sail,
            AbilityName::Socialize => Self::Socialize,
            AbilityName::Stealth => Self::Stealth,
            AbilityName::Survival => Self::Survival,
            AbilityName::Thrown => Self::Thrown,
            AbilityName::War => Self::War,
        }
    }
}

impl From<AbilityNameVanilla> for AbilityName<'_> {
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

impl From<AbilityNameVanilla> for AbilityNameNoSubskill {
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

impl TryFrom<AbilityNameNoSubskill> for AbilityNameVanilla {
    type Error = eyre::Report;

    fn try_from(value: AbilityNameNoSubskill) -> Result<Self, Self::Error> {
        match value {
            AbilityNameNoSubskill::Archery => Ok(Self::Archery),
            AbilityNameNoSubskill::Athletics => Ok(Self::Athletics),
            AbilityNameNoSubskill::Awareness => Ok(Self::Awareness),
            AbilityNameNoSubskill::Brawl => Ok(Self::Brawl),
            AbilityNameNoSubskill::Bureaucracy => Ok(Self::Bureaucracy),
            AbilityNameNoSubskill::Craft => Err(eyre!("Craft requires a focus")),
            AbilityNameNoSubskill::Dodge => Ok(Self::Dodge),
            AbilityNameNoSubskill::Integrity => Ok(Self::Integrity),
            AbilityNameNoSubskill::Investigation => Ok(Self::Investigation),
            AbilityNameNoSubskill::Larceny => Ok(Self::Larceny),
            AbilityNameNoSubskill::Linguistics => Ok(Self::Linguistics),
            AbilityNameNoSubskill::Lore => Ok(Self::Lore),
            AbilityNameNoSubskill::MartialArts => Err(eyre!("Martial Arts requires a style")),
            AbilityNameNoSubskill::Medicine => Ok(Self::Medicine),
            AbilityNameNoSubskill::Melee => Ok(Self::Melee),
            AbilityNameNoSubskill::Occult => Ok(Self::Occult),
            AbilityNameNoSubskill::Performance => Ok(Self::Performance),
            AbilityNameNoSubskill::Presence => Ok(Self::Presence),
            AbilityNameNoSubskill::Resistance => Ok(Self::Resistance),
            AbilityNameNoSubskill::Ride => Ok(Self::Ride),
            AbilityNameNoSubskill::Sail => Ok(Self::Sail),
            AbilityNameNoSubskill::Socialize => Ok(Self::Socialize),
            AbilityNameNoSubskill::Stealth => Ok(Self::Stealth),
            AbilityNameNoSubskill::Survival => Ok(Self::Survival),
            AbilityNameNoSubskill::Thrown => Ok(Self::Thrown),
            AbilityNameNoSubskill::War => Ok(Self::War),
        }
    }
}
