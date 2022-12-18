mod dawn;
mod eclipse;
mod night;
mod twilight;
mod zenith;

pub use self::{
    dawn::{DawnAbility, DawnTraits, DawnTraitsBuilder},
    eclipse::{EclipseAbility, EclipseTraits, EclipseTraitsBuilder},
    night::{NightAbility, NightTraits, NightTraitsBuilder},
    twilight::{TwilightAbility, TwilightTraits, TwilightTraitsBuilder},
    zenith::{ZenithAbility, ZenithTraits, ZenithTraitsBuilder},
};

use crate::{
    abilities::AbilityNameNoSubskill, anima::AnimaLevel, charms::SolarCharm, essence::Essence,
    limit::Limit, sorcery::SolarSorcererLevel,
};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarTraits {
    pub essence: Essence,
    pub limit: Limit,
    pub anima: AnimaLevel,
    caste: SolarCaste,
    favored_abilities: [AbilityNameNoSubskill; 5],
    sorcery_level: SolarSorcererLevel,
    solar_charms: Vec<SolarCharm>,
}

impl SolarTraits {
    /// Brawl and MartialArts are different supernal abilities for Dawn castes.
    pub fn supernal_ability(&self) -> AbilityNameNoSubskill {
        self.caste.supernal_ability()
    }

    /// Brawl and MartialArts are different supernal abilities for Dawn castes.
    pub fn has_supernal_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        self.caste.has_supernal_ability(ability)
    }

    /// Brawl implies Brawl/MartialArts here.
    pub fn caste_abilities(&self) -> [AbilityNameNoSubskill; 5] {
        self.caste.caste_abilities()
    }

    /// Brawl implies Brawl/MartialArts here.
    pub fn favored_abilities(&self) -> [AbilityNameNoSubskill; 5] {
        self.favored_abilities
    }

    /// Brawl implies Brawl/MartialArts.
    pub fn caste_and_favored_abilities(&self) -> [AbilityNameNoSubskill; 10] {
        let mut output: [AbilityNameNoSubskill; 10] =
            [self.caste_abilities(), self.favored_abilities()]
                .into_iter()
                .flat_map(|arr| arr.into_iter())
                .enumerate()
                .fold(
                    [AbilityNameNoSubskill::Archery; 10],
                    |mut arr, (index, ability)| {
                        arr[index] = ability;
                        arr
                    },
                );
        output.sort();
        output
    }

    /// Returns true for MartialArts if Brawl is a caste ability.
    pub fn has_caste_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        self.caste.has_caste_ability(ability)
    }

    /// Returns true for MartialArts if Brawl is a favored ability.
    pub fn has_favored_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        self.favored_abilities.contains(&ability)
    }

    /// Returns true for MartialArts if Brawl is a caste or favored ability.
    pub fn has_caste_or_favored_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        self.has_favored_ability(ability) || self.has_caste_ability(ability)
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

impl SolarCaste {
    fn supernal_ability(&self) -> AbilityNameNoSubskill {
        match &self {
            SolarCaste::Dawn(traits) => traits.supernal_ability(),
            SolarCaste::Zenith(traits) => traits.supernal_ability(),
            SolarCaste::Twilight(traits) => traits.supernal_ability(),
            SolarCaste::Night(traits) => traits.supernal_ability(),
            SolarCaste::Eclipse(traits) => traits.supernal_ability(),
        }
    }

    fn caste_abilities(&self) -> [AbilityNameNoSubskill; 5] {
        match &self {
            SolarCaste::Dawn(traits) => traits.caste_abilities(),
            SolarCaste::Zenith(traits) => traits.caste_abilities(),
            SolarCaste::Twilight(traits) => traits.caste_abilities(),
            SolarCaste::Night(traits) => traits.caste_abilities(),
            SolarCaste::Eclipse(traits) => traits.caste_abilities(),
        }
    }

    /// Brawl and MartialArts are considered different supernal abilities
    fn has_supernal_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        match &self {
            SolarCaste::Dawn(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Zenith(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Twilight(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Night(traits) => traits.has_supernal_ability(ability),
            SolarCaste::Eclipse(traits) => traits.has_supernal_ability(ability),
        }
    }

    fn has_caste_ability(&self, ability: AbilityNameNoSubskill) -> bool {
        match &self {
            SolarCaste::Dawn(traits) => traits.has_caste_ability(ability),
            SolarCaste::Zenith(traits) => traits.has_caste_ability(ability),
            SolarCaste::Twilight(traits) => traits.has_caste_ability(ability),
            SolarCaste::Night(traits) => traits.has_caste_ability(ability),
            SolarCaste::Eclipse(traits) => traits.has_caste_ability(ability),
        }
    }
}

pub struct SolarTraitsBuilder {
    essence: Essence,
    limit: Option<Limit>,
    anima: AnimaLevel,
    caste: Option<SolarCaste>,
    favored: Vec<AbilityNameNoSubskill>,
    sorcery_level: SolarSorcererLevel,
    solar_charms: Vec<SolarCharm>,
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

    pub fn with_anima_level(mut self, anima_level: AnimaLevel) -> Self {
        self.anima = anima_level;
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
        if ability == AbilityNameNoSubskill::MartialArts {
            Err(eyre!(
                "Martial Arts cannot be chosen as a favored ability; it comes for free with Brawl"
            ))
        } else {
            self.favored.push(ability);
            Ok(self)
        }
    }

    pub fn with_solar_charm_unchecked(mut self, charm: SolarCharm) -> Self {
        self.solar_charms.push(charm);
        self
    }

    pub fn build(mut self) -> Result<SolarTraits> {
        if self.caste.is_none() {
            return Err(eyre!("Solars must have a caste"));
        }

        if self.limit.is_none() {
            return Err(eyre!("Solars must have a limit trigger"));
        }

        self.favored.sort();
        self.favored.dedup();
        let caste = self.caste.unwrap();
        let caste_abilities = caste.caste_abilities();

        self.favored
            .retain(|ability| !caste_abilities.contains(ability));
        if self.favored.len() != 5 {
            Err(eyre!("Solars must have a total of 10 caste and favored abilities (not counting Martial Arts)"))
        } else {
            let favored_abilities = self.favored.into_iter().enumerate().fold(
                [AbilityNameNoSubskill::Archery; 5],
                |mut arr, (index, ability)| {
                    arr[index] = ability;
                    arr
                },
            );

            Ok(SolarTraits {
                essence: self.essence,
                limit: self.limit.unwrap(),
                anima: self.anima,
                caste,
                favored_abilities,
                sorcery_level: self.sorcery_level,
                solar_charms: self.solar_charms,
            })
        }
    }
}
