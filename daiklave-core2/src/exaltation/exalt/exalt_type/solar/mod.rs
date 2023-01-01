/// Traits relating to specific Solar castes.
pub mod caste;

mod builder;
mod builder_error;
mod exalt;
mod solar;

pub use solar::Solar;

use crate::{
    abilities::AbilityName,
    guided::ExaltationChoice,
    sorcery::{SolarSorcererView},
};

use self::caste::SolarCasteView;

/// Traits which are unique to being a Solar Exalted, with &str
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolarView<'source> {
    caste: SolarCasteView,
    favored_abilities: [AbilityName; 5],
    pub(crate) sorcery: Option<SolarSorcererView<'source>>,
}

impl<'source> SolarView<'source> {
    /// Returns True if the ability is a caste ability for the charcter. Note
    /// that MartialArts is a caste ability if and only if Brawl is a caste
    /// ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        self.caste.has_caste_ability(ability)
    }

    /// Returns the Solar's supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        self.caste.supernal_ability()
    }

    /// Returns True if the ability is a favored ability for the charcter. Note
    /// that MartialArts is a favored ability if and only if Brawl is a favored
    /// ability.
    pub fn has_favored_ability(&self, ability: AbilityName) -> bool {
        self.favored_abilities.iter().any(|&a| a == ability)
    }
}

pub(crate) fn validate_solar_caste_ability(
    exaltation: ExaltationChoice,
    ability: AbilityName,
) -> bool {
    matches!(
        (exaltation, ability),
        (ExaltationChoice::Dawn, AbilityName::Archery)
            | (ExaltationChoice::Dawn, AbilityName::Awareness)
            | (ExaltationChoice::Dawn, AbilityName::Brawl)
            | (ExaltationChoice::Dawn, AbilityName::Dodge)
            | (ExaltationChoice::Dawn, AbilityName::Melee)
            | (ExaltationChoice::Dawn, AbilityName::Resistance)
            | (ExaltationChoice::Dawn, AbilityName::Thrown)
            | (ExaltationChoice::Dawn, AbilityName::War)
            | (ExaltationChoice::Zenith, AbilityName::Athletics)
            | (ExaltationChoice::Zenith, AbilityName::Integrity)
            | (ExaltationChoice::Zenith, AbilityName::Performance)
            | (ExaltationChoice::Zenith, AbilityName::Lore)
            | (ExaltationChoice::Zenith, AbilityName::Presence)
            | (ExaltationChoice::Zenith, AbilityName::Resistance)
            | (ExaltationChoice::Zenith, AbilityName::Survival)
            | (ExaltationChoice::Zenith, AbilityName::War)
            | (ExaltationChoice::Twilight, AbilityName::Bureaucracy)
            | (ExaltationChoice::Twilight, AbilityName::Craft)
            | (ExaltationChoice::Twilight, AbilityName::Integrity)
            | (ExaltationChoice::Twilight, AbilityName::Investigation)
            | (ExaltationChoice::Twilight, AbilityName::Linguistics)
            | (ExaltationChoice::Twilight, AbilityName::Lore)
            | (ExaltationChoice::Twilight, AbilityName::Medicine)
            | (ExaltationChoice::Twilight, AbilityName::Occult)
            | (ExaltationChoice::Night, AbilityName::Athletics)
            | (ExaltationChoice::Night, AbilityName::Awareness)
            | (ExaltationChoice::Night, AbilityName::Dodge)
            | (ExaltationChoice::Night, AbilityName::Investigation)
            | (ExaltationChoice::Night, AbilityName::Larceny)
            | (ExaltationChoice::Night, AbilityName::Ride)
            | (ExaltationChoice::Night, AbilityName::Stealth)
            | (ExaltationChoice::Night, AbilityName::Socialize)
            | (ExaltationChoice::Eclipse, AbilityName::Bureaucracy)
            | (ExaltationChoice::Eclipse, AbilityName::Larceny)
            | (ExaltationChoice::Eclipse, AbilityName::Linguistics)
            | (ExaltationChoice::Eclipse, AbilityName::Occult)
            | (ExaltationChoice::Eclipse, AbilityName::Presence)
            | (ExaltationChoice::Eclipse, AbilityName::Ride)
            | (ExaltationChoice::Eclipse, AbilityName::Sail)
            | (ExaltationChoice::Eclipse, AbilityName::Socialize)
    )
}
