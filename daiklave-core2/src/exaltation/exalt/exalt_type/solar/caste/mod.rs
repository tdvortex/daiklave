pub(crate) mod dawn;
pub(crate) mod eclipse;
pub(crate) mod night;
pub(crate) mod twilight;
pub(crate) mod zenith;

pub use dawn::{DawnCasteAbility, DawnSupernalAbility};
pub use eclipse::EclipseAbility;
pub use night::NightAbility;
pub use twilight::TwilightAbility;
pub use zenith::ZenithAbility;

mod memo;

pub(crate) use memo::SolarCasteMemo;

use crate::{abilities::AbilityName, exaltation::exalt::AnimaEffect};

use self::{dawn::Dawn, eclipse::Eclipse, night::Night, twilight::Twilight, zenith::Zenith};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SolarCaste<'source> {
    Dawn(Dawn),
    Zenith(Zenith),
    Twilight(Twilight),
    Night(Night),
    Eclipse(Eclipse<'source>),
}

impl<'source> SolarCaste<'source> {
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

    pub fn anima_effects(&self) -> [AnimaEffect<'static>; 3] {
        match self {
            SolarCaste::Dawn(dawn) => dawn.anima_effects(),
            SolarCaste::Zenith(zenith) => zenith.anima_effects(),
            SolarCaste::Twilight(twilight) => twilight.anima_effects(),
            SolarCaste::Night(night) => night.anima_effects(),
            SolarCaste::Eclipse(eclipse) => eclipse.anima_effects(),
        }
    }
}
