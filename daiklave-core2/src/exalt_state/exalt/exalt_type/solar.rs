use std::collections::HashSet;

use serde::{Deserialize, Serialize};

mod builder;
mod caste;
mod caste_view;
mod character;
mod character_view;
mod dawn;
mod eclipse;
mod essence;
mod exalt;
mod night;
mod twilight;
mod zenith;

pub use dawn::{Dawn, DawnBuilder};
pub use eclipse::{Eclipse, EclipseBuilder};
pub use night::{Night, NightBuilder};
pub use twilight::{Twilight, TwilightBuilder};
pub use zenith::{Zenith, ZenithBuilder};

use crate::{
    abilities::AbilityName,
    guided::ExaltationChoice,
    sorcery::{SolarSorcerer, SolarSorcererView},
};

use self::{
    builder::SolarTraitsBuilder, caste::SolarCaste, caste_view::SolarCasteView, dawn::DawnView,
    eclipse::EclipseView, night::NightView, twilight::TwilightView, zenith::ZenithView,
};

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solar {
    caste: SolarCaste,
    favored_abilities: [AbilityName; 5],
    pub(crate) sorcery: Option<SolarSorcerer>,
}

impl<'source> Solar {
    /// Creates a builder to construct SolarTraits.
    pub fn builder() -> SolarTraitsBuilder<'source> {
        SolarTraitsBuilder {
            caste: None,
            favored_abilities: HashSet::new(),
            sorcery: None,
        }
    }

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

impl<'char> Solar {
    pub(crate) fn as_view(&'char self) -> SolarView<'char> {
        let caste = match &self.caste {
            SolarCaste::Dawn(dawn) => SolarCasteView::Dawn(DawnView {
                caste_not_supernal: dawn.caste_not_supernal,
                supernal: dawn.supernal,
            }),
            SolarCaste::Zenith(zenith) => SolarCasteView::Zenith(ZenithView {
                caste_not_supernal: zenith.caste_not_supernal,
                supernal: zenith.supernal,
            }),
            SolarCaste::Twilight(twilight) => SolarCasteView::Twilight(TwilightView {
                caste_not_supernal: twilight.caste_not_supernal,
                supernal: twilight.supernal,
            }),
            SolarCaste::Night(night) => SolarCasteView::Night(NightView {
                caste_not_supernal: night.caste_not_supernal,
                supernal: night.supernal,
            }),
            SolarCaste::Eclipse(eclipse) => SolarCasteView::Eclipse(EclipseView {
                caste_not_supernal: eclipse.caste_not_supernal,
                supernal: eclipse.supernal,
            }),
        };
        let favored_abilities = self.favored_abilities;
        let sorcery = self.sorcery.as_ref().map(|sorcery| sorcery.as_view());

        SolarView {
            caste,
            favored_abilities,
            sorcery,
        }
    }
}

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
