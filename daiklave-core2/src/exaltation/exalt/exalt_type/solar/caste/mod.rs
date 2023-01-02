/// Dawn Caste, warriors and generals
pub mod dawn;

/// Eclipse Caste, dealmakers and travelers
pub mod eclipse;

/// Night Caste, rogues and swashbucklers
pub mod night;

/// Twilight Caste, scholars and sorcerers
pub mod twilight;

/// Zenith Caste, leaders and priests
pub mod zenith;

mod caste_memo;

pub(crate) use caste_memo::SolarCasteMemo;

use crate::abilities::AbilityName;

use self::{dawn::Dawn, eclipse::Eclipse, night::Night, twilight::Twilight, zenith::Zenith};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SolarCaste {
    Dawn(Dawn),
    Zenith(Zenith),
    Twilight(Twilight),
    Night(Night),
    Eclipse(Eclipse),
}

impl SolarCaste {
    pub(crate) fn as_memo(&self) -> SolarCasteMemo {
        match self {
            SolarCaste::Dawn(view) => SolarCasteMemo::Dawn(view.as_memo()),
            SolarCaste::Zenith(view) => SolarCasteMemo::Zenith(view.as_memo()),
            SolarCaste::Twilight(view) => SolarCasteMemo::Twilight(view.as_memo()),
            SolarCaste::Night(view) => SolarCasteMemo::Night(view.as_memo()),
            SolarCaste::Eclipse(view) => SolarCasteMemo::Eclipse(view.as_memo()),
        }
    }

    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        match self {
            SolarCaste::Dawn(dawn) => dawn.has_caste_ability(ability),
            SolarCaste::Zenith(zenith) => zenith.has_caste_ability(ability),
            SolarCaste::Twilight(twilight) => twilight.has_caste_ability(ability),
            SolarCaste::Night(night) => night.has_caste_ability(ability),
            SolarCaste::Eclipse(eclipse) => eclipse.has_caste_ability(ability),
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        match self {
            SolarCaste::Dawn(dawn) => dawn.supernal_ability(),
            SolarCaste::Zenith(zenith) => zenith.supernal_ability(),
            SolarCaste::Twilight(twilight) => twilight.supernal_ability(),
            SolarCaste::Night(night) => night.supernal_ability(),
            SolarCaste::Eclipse(eclipse) => eclipse.supernal_ability(),
        }
    }
}
