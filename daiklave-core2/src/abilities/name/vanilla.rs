use crate::exaltation::exalt::exalt_type::solar::charm::SolarCharmAbility;

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

impl TryFrom<SolarCharmAbility> for AbilityNameVanilla {
    type Error = ();

    fn try_from(value: SolarCharmAbility) -> Result<Self, Self::Error> {
        match value {
            SolarCharmAbility::Archery => Ok(Self::Archery),
            SolarCharmAbility::Athletics => Ok(Self::Athletics),
            SolarCharmAbility::Awareness => Ok(Self::Awareness),
            SolarCharmAbility::Brawl => Ok(Self::Brawl),
            SolarCharmAbility::Bureaucracy => Ok(Self::Bureaucracy),
            SolarCharmAbility::Craft => Err(()),
            SolarCharmAbility::Dodge => Ok(Self::Dodge),
            SolarCharmAbility::Integrity => Ok(Self::Integrity),
            SolarCharmAbility::Investigation => Ok(Self::Investigation),
            SolarCharmAbility::Larceny => Ok(Self::Larceny),
            SolarCharmAbility::Linguistics => Ok(Self::Linguistics),
            SolarCharmAbility::Lore => Ok(Self::Lore),
            SolarCharmAbility::Medicine => Ok(Self::Medicine),
            SolarCharmAbility::Melee => Ok(Self::Melee),
            SolarCharmAbility::Occult => Ok(Self::Occult),
            SolarCharmAbility::Performance => Ok(Self::Performance),
            SolarCharmAbility::Presence => Ok(Self::Presence),
            SolarCharmAbility::Resistance => Ok(Self::Resistance),
            SolarCharmAbility::Ride => Ok(Self::Ride),
            SolarCharmAbility::Sail => Ok(Self::Sail),
            SolarCharmAbility::Socialize => Ok(Self::Socialize),
            SolarCharmAbility::Stealth => Ok(Self::Stealth),
            SolarCharmAbility::Survival => Ok(Self::Survival),
            SolarCharmAbility::Thrown => Ok(Self::Thrown),
            SolarCharmAbility::War => Ok(Self::War),
        }
    }
}
