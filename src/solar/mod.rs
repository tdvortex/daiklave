mod dawn;
mod eclipse;
mod night;
mod twilight;
mod zenith;

pub use self::{
    dawn::{DawnAbility, DawnTraits, DawnTraitsBuilder},
    twilight::{TwilightAbility, TwilightTraits, TwilightTraitsBuilder},
    zenith::{ZenithAbility, ZenithTraits, ZenithTraitsBuilder},
    night::{NightAbility, NightTraits, NightTraitsBuilder},
    eclipse::{EclipseAbility, EclipseTraits, EclipseTraitsBuilder},
};

use crate::{abilities::AbilityNameNoSubskill, essence::Essence, limit::Limit};
use eyre::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarTraits {
    pub essence: Essence,
    pub limit: Limit,
    caste: SolarCaste,
    favored_abilities: Vec<AbilityNameNoSubskill>,
}

impl SolarTraits {
    pub fn supernal_ability(&self) -> AbilityNameNoSubskill {
        match &self.caste {
            SolarCaste::Dawn(traits) => traits.supernal_ability(),
            SolarCaste::Zenith(traits) => traits.supernal_ability(),
            SolarCaste::Twilight(traits) => traits.supernal_ability(),
            SolarCaste::Night(traits) => traits.supernal_ability(),
            SolarCaste::Eclipse(traits) => traits.supernal_ability(),
        }
    }

    pub fn caste_abilities(&self) -> Vec<AbilityNameNoSubskill> {
        match &self.caste {
            SolarCaste::Dawn(traits) => traits.caste_abilities(),
            SolarCaste::Zenith(traits) => traits.caste_abilities(),
            SolarCaste::Twilight(traits) => traits.caste_abilities(),
            SolarCaste::Night(traits) => traits.caste_abilities(),
            SolarCaste::Eclipse(traits) => traits.caste_abilities(),
        }
    }

    pub fn has_supernal_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        match &self.caste {
            SolarCaste::Dawn(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Zenith(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Twilight(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Night(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Eclipse(traits) => traits.has_supernal_ability(ability),
        }
    }

    pub fn has_caste_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        match &self.caste {
            SolarCaste::Dawn(traits) => traits.has_caste_ability(ability),
            SolarCaste::Zenith(traits) => traits.has_caste_ability(ability),
            SolarCaste::Twilight(traits) => traits.has_caste_ability(ability),
            SolarCaste::Night(traits) => traits.has_caste_ability(ability),
            SolarCaste::Eclipse(traits) => traits.has_caste_ability(ability),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SolarCaste {
    Dawn(DawnTraits),
    Zenith(ZenithTraits),
    Twilight(TwilightTraits),
    Night(NightTraits),
    Eclipse(EclipseTraits),
}

pub struct SolarTraitsBuilder {
    essence: Essence,
    limit: Option<Limit>,
    caste: Option<SolarCaste>,
    favored: Vec<AbilityNameNoSubskill>,
}

impl SolarTraitsBuilder {
    pub fn with_essence_rating(mut self, rating: u8) -> Result<Self> {
        self.essence = Essence::solar(rating)?;
        Ok(self)
    }

    pub fn with_limit(mut self, limit_trigger: String, track: u8) -> Self {
        self.limit = Some(Limit {
            track,
            limit_trigger,
        });
        self
    }

    pub fn as_dawn(mut self, dawn_traits: DawnTraits) -> Self {
        self.caste = Some(SolarCaste::Dawn(dawn_traits));
        self
    }

    pub fn as_zenith(mut self, zenith_traits: ZenithTraits) -> Self {
        self.caste = Some(SolarCaste::Zenith(zenith_traits));
        self
    }

    pub fn as_twilight(mut self, twilight_traits: TwilightTraits) -> Self {
        self.caste = Some(SolarCaste::Twilight(twilight_traits));
        self
    }

    pub fn as_night(mut self, night_traits: NightTraits) -> Self {
        self.caste = Some(SolarCaste::Night(night_traits));
        self
    }

    pub fn as_eclipse(mut self, eclipse_traits: EclipseTraits) -> Self {
        self.caste = Some(SolarCaste::Eclipse(eclipse_traits));
        self
    }

    pub fn with_favored_ability(mut self, ability: AbilityNameNoSubskill) -> Result<Self> {
        todo!()
    }
}
