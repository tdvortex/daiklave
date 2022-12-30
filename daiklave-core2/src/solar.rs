use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    essence::{Essence, EssenceView, MoteCommitmentView, MoteState, MotesView},
    exalt_type::{ExaltState, ExaltStateView, ExaltType, ExaltTypeView},
    guided::ExaltationChoice,
    AbilityName, CharacterMutationError, CommittedMotesId,
};

use self::{
    builder::SolarTraitsBuilder, character::SolarCaste, character_view::SolarCasteView,
    dawn::DawnView, eclipse::EclipseView, night::NightView, twilight::TwilightView,
    zenith::ZenithView,
};
mod builder;
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

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solar {
    pub(crate) essence: Essence,
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
pub struct SolarView<'source> {
    pub(crate) essence: EssenceView<'source>,
    caste: SolarCasteView,
    favored_abilities: [AbilityName; 5],
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

impl<'source> ExaltTypeView<'source> {
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
        if let Self::Exalted(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(&self, _solar_traits: &Solar) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(&mut self, solar_traits: &Solar) -> Result<&mut Self, CharacterMutationError> {
        *self = Self::Exalted(ExaltType::Solar(solar_traits.clone()));
        Ok(self)
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
        _solar_traits: &'source Solar,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar_traits: &'source Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        let rating = solar_traits.essence.rating();

        let peripheral = {
            let available = solar_traits.essence.motes().peripheral().available();
            let spent = solar_traits.essence.motes().peripheral().spent();
            MoteState { available, spent }
        };

        let personal = {
            let available = solar_traits.essence.motes().personal().available();
            let spent = solar_traits.essence.motes().personal().spent();
            MoteState { available, spent }
        };

        let commitments = solar_traits
            .essence
            .motes()
            .committed()
            .map(|(id, name, peripheral, personal)| {
                (
                    id,
                    MoteCommitmentView {
                        name,
                        peripheral,
                        personal,
                    },
                )
            })
            .collect::<HashMap<CommittedMotesId, MoteCommitmentView>>();

        let motes = MotesView {
            peripheral,
            personal,
            commitments,
        };

        let essence = EssenceView { rating, motes };

        let caste = match &solar_traits.caste {
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
        let favored_abilities = solar_traits.favored_abilities;

        let solar_traits_view = SolarView {
            essence,
            caste,
            favored_abilities,
        };

        *self = Self::Exalted(ExaltTypeView::Solar(solar_traits_view));
        Ok(self)
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
