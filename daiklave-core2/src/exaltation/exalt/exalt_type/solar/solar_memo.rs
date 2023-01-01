use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityName};

use super::{
    builder::SolarBuilder,
    caste::{
        dawn::DawnView, eclipse::EclipseView, night::NightView, twilight::TwilightView,
        zenith::ZenithView, SolarCasteMemo, SolarCasteView,
    },
    solar_view::SolarView, sorcery::SolarSorcererMemo,
};

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarMemo {
    caste: SolarCasteMemo,
    favored_abilities: [AbilityName; 5],
    pub(crate) sorcery: Option<SolarSorcererMemo>,
}

impl<'source> SolarMemo {
    pub(crate) fn new(
        caste: SolarCasteMemo,
        favored_abilities: [AbilityName; 5],
        sorcery: Option<SolarSorcererMemo>,
    ) -> Self {
        Self {
            caste,
            favored_abilities,
            sorcery,
        }
    }

    /// Creates a builder to construct SolarTraits.
    pub fn builder() -> SolarBuilder<'source> {
        SolarBuilder::default()
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

impl<'char> SolarMemo {
    pub(crate) fn as_view(&'char self) -> SolarView<'char> {
        let caste = match &self.caste {
            SolarCasteMemo::Dawn(dawn) => SolarCasteView::Dawn(DawnView {
                caste_not_supernal: dawn.caste_not_supernal,
                supernal: dawn.supernal,
            }),
            SolarCasteMemo::Zenith(zenith) => SolarCasteView::Zenith(ZenithView {
                caste_not_supernal: zenith.caste_not_supernal,
                supernal: zenith.supernal,
            }),
            SolarCasteMemo::Twilight(twilight) => SolarCasteView::Twilight(TwilightView {
                caste_not_supernal: twilight.caste_not_supernal,
                supernal: twilight.supernal,
            }),
            SolarCasteMemo::Night(night) => SolarCasteView::Night(NightView {
                caste_not_supernal: night.caste_not_supernal,
                supernal: night.supernal,
            }),
            SolarCasteMemo::Eclipse(eclipse) => SolarCasteView::Eclipse(EclipseView {
                caste_not_supernal: eclipse.caste_not_supernal,
                supernal: eclipse.supernal,
            }),
        };
        let favored_abilities = self.favored_abilities;
        let sorcery = self.sorcery.as_ref().map(|sorcery| sorcery.as_view());

        SolarView::new(caste, favored_abilities, sorcery)
    }

    pub(crate) fn sorcery(&self) -> Option<&SolarSorcererMemo> {
        self.sorcery.as_ref()
    }
}