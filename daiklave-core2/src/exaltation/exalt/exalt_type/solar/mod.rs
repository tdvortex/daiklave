/// Traits relating to specific Solar castes.
pub mod caste;

mod builder;
mod builder_error;
mod exalt;
mod solar;
mod solar_view;

pub use solar::Solar;
pub use solar_view::SolarView;

use crate::{abilities::AbilityName, guided::ExaltationChoice};

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
