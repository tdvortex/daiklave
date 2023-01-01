use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::{dawn::DawnMemo, eclipse::EclipseMemo, night::NightMemo, twilight::TwilightMemo, zenith::ZenithMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolarCasteMemo {
    Dawn(DawnMemo),
    Zenith(ZenithMemo),
    Twilight(TwilightMemo),
    Night(NightMemo),
    Eclipse(EclipseMemo),
}

impl SolarCasteMemo {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        match self {
            SolarCasteMemo::Dawn(dawn) => dawn.has_caste_ability(ability),
            SolarCasteMemo::Zenith(zenith) => zenith.has_caste_ability(ability),
            SolarCasteMemo::Twilight(twilight) => twilight.has_caste_ability(ability),
            SolarCasteMemo::Night(night) => night.has_caste_ability(ability),
            SolarCasteMemo::Eclipse(eclipse) => eclipse.has_caste_ability(ability),
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        match self {
            SolarCasteMemo::Dawn(dawn) => dawn.supernal_ability(),
            SolarCasteMemo::Zenith(zenith) => zenith.supernal_ability(),
            SolarCasteMemo::Twilight(twilight) => twilight.supernal_ability(),
            SolarCasteMemo::Night(night) => night.supernal_ability(),
            SolarCasteMemo::Eclipse(eclipse) => eclipse.supernal_ability(),
        }
    }
}
