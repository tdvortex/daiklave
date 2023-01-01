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

impl AbilityNameVanilla {
    /// Iterates over all ability names except for Craft and Martial Arts,
    /// in alphabetical order.
    pub fn iter() -> impl Iterator<Item = AbilityNameVanilla> {
        [
            Self::Archery,
            Self::Athletics,
            Self::Awareness,
            Self::Brawl,
            Self::Bureaucracy,
            Self::Dodge,
            Self::Integrity,
            Self::Investigation,
            Self::Larceny,
            Self::Linguistics,
            Self::Lore,
            Self::Medicine,
            Self::Melee,
            Self::Occult,
            Self::Performance,
            Self::Presence,
            Self::Resistance,
            Self::Ride,
            Self::Sail,
            Self::Socialize,
            Self::Stealth,
            Self::Survival,
            Self::Thrown,
            Self::War,
        ]
        .into_iter()
    }
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

impl TryFrom<AbilityName> for AbilityNameVanilla {
    type Error = ();

    fn try_from(value: AbilityName) -> Result<Self, Self::Error> {
        match value {
            AbilityName::Craft => Err(()),
            AbilityName::MartialArts => Err(()),
            AbilityName::Archery => Ok(Self::Archery),
            AbilityName::Athletics => Ok(Self::Athletics),
            AbilityName::Awareness => Ok(Self::Awareness),
            AbilityName::Brawl => Ok(Self::Brawl),
            AbilityName::Bureaucracy => Ok(Self::Bureaucracy),
            AbilityName::Dodge => Ok(Self::Dodge),
            AbilityName::Integrity => Ok(Self::Integrity),
            AbilityName::Investigation => Ok(Self::Investigation),
            AbilityName::Larceny => Ok(Self::Larceny),
            AbilityName::Linguistics => Ok(Self::Linguistics),
            AbilityName::Lore => Ok(Self::Lore),
            AbilityName::Medicine => Ok(Self::Medicine),
            AbilityName::Melee => Ok(Self::Melee),
            AbilityName::Occult => Ok(Self::Occult),
            AbilityName::Performance => Ok(Self::Performance),
            AbilityName::Presence => Ok(Self::Presence),
            AbilityName::Resistance => Ok(Self::Resistance),
            AbilityName::Ride => Ok(Self::Ride),
            AbilityName::Sail => Ok(Self::Sail),
            AbilityName::Socialize => Ok(Self::Socialize),
            AbilityName::Stealth => Ok(Self::Stealth),
            AbilityName::Survival => Ok(Self::Survival),
            AbilityName::Thrown => Ok(Self::Thrown),
            AbilityName::War => Ok(Self::War),
        }
    }
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
