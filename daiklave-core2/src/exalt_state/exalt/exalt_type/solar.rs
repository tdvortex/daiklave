use std::collections::{HashSet};

use serde::{Deserialize, Serialize};


mod builder;
mod caste;
mod caste_view;
mod character;
mod character_view;
mod dawn;
mod eclipse;
mod night;
mod twilight;
mod zenith;

pub use dawn::{Dawn, DawnBuilder};
pub use eclipse::{Eclipse, EclipseBuilder};
pub use night::{Night, NightBuilder};
pub use twilight::{Twilight, TwilightBuilder};
pub use zenith::{Zenith, ZenithBuilder};

use crate::{exalt_state::{ExaltStateView, ExaltState, exalt::{Exalt, ExaltView}}, guided::ExaltationChoice, CharacterMutationError, abilities::AbilityName};

use self::{builder::SolarTraitsBuilder, night::NightView, twilight::TwilightView, eclipse::EclipseView, dawn::DawnView, zenith::ZenithView, caste::SolarCaste, caste_view::SolarCasteView};

use super::{ExaltType, ExaltTypeView};

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solar {
    caste: SolarCaste,
    favored_abilities: [AbilityName; 5],
}

impl Solar {
    /// Creates a builder to construct SolarTraits.
    pub fn builder() -> SolarTraitsBuilder {
        SolarTraitsBuilder {
            caste: None,
            favored_abilities: HashSet::new(),
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

/// Traits which are unique to being a Solar Exalted, with &str
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolarView {
    caste: SolarCasteView,
    favored_abilities: [AbilityName; 5],
}

impl SolarView {
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

impl ExaltType {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        match self {
            ExaltType::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl ExaltTypeView {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        match self {
            ExaltTypeView::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl ExaltState {
    pub fn is_solar(&self) -> bool {
        if let Self::Exalt(exalt) = self {
            exalt.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        if let Self::Exalt(exalt) = self {
            exalt.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(&self, _solar: &Solar) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(&mut self, solar: &Solar) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_solar(&self) -> bool {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(
        &self,
        _solar: &'source Solar,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar: &'source Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        let caste = match &solar.caste {
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
        let favored_abilities = solar.favored_abilities;

        let solar_traits_view = SolarView {
            caste,
            favored_abilities,
        };

        todo!()
    }
}

impl Exalt {
    pub fn is_solar(&self) -> bool {
        self.exalt_type.is_solar()
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        self.exalt_type.solar_traits()
    }
}


impl<'source> ExaltView<'source> {
    pub fn is_solar(&self) -> bool {
        self.exalt_type.is_solar()
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        self.exalt_type.solar_traits()
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
