use serde::{Deserialize, Serialize};

use crate::{exalt_state::exalt::exalt_type::solar::{Dawn, Zenith, Twilight, Night, Eclipse}, abilities::AbilityName};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolarCaste {
    Dawn(Dawn),
    Zenith(Zenith),
    Twilight(Twilight),
    Night(Night),
    Eclipse(Eclipse),
}

impl SolarCaste {
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
