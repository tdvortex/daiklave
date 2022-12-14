use crate::{abilities::AbilityNameNoSubskill, essence::Essence, limit::Limit};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarTraits {
    pub essence: Essence,
    pub limit: Limit,
    caste: SolarCaste,
    favored_abilities: Vec<AbilityNameNoSubskill>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SolarCaste {
    Dawn(DawnTraits),
    Zenith(ZenithTraits),
    Twilight(TwilightTraits),
    Night(NightTraits),
    Eclipse(EclipseTraits),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum DawnAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    Melee,
    Resistance,
    Thrown,
    War,
}

/// Dawn Solars can't choose MartialArts as a caste ability, but can choose it
/// as their Supernal if and only if Brawl is one of their five caste abilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DawnTraits {
    MartialArtsSupernal(Vec<DawnAbility>),
    NotMartialArtsSupernal(DawnAbility, Vec<DawnAbility>),
}

impl DawnTraits {
    pub fn builder() -> DawnTraitsBuilder {
        DawnTraitsBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct DawnTraitsBuilder {
    is_martial_arts_supernal: bool,
    supernal_ability: Option<DawnAbility>,
    caste_abilities: Vec<DawnAbility>,
}

impl DawnTraitsBuilder {
    pub fn with_non_supernal_caste_ability(mut self, ability: DawnAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn with_supernal_ability(mut self, ability: DawnAbility) -> Self {
        if self.is_martial_arts_supernal {
            self.is_martial_arts_supernal = false;
        }
        self.is_martial_arts_supernal = false;
        self.caste_abilities.retain(|a| *a != ability);
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_martial_arts_supernal(mut self) -> Self {
        self.is_martial_arts_supernal = true;
        self.supernal_ability = None;
        self.caste_abilities.retain(|a| *a != DawnAbility::Brawl);
        self
    }

    pub fn build(mut self) -> Result<DawnTraits> {
        if self.is_martial_arts_supernal && self.supernal_ability.is_some() {
            return Err(eyre!("Cannot have multiple supernal abilities"));
        }

        if !self.is_martial_arts_supernal && self.supernal_ability.is_none() {
            return Err(eyre!("Must specify a supernal ability"));
        }

        self.caste_abilities.sort();
        self.caste_abilities.dedup();

        if self.is_martial_arts_supernal {
            self.caste_abilities.retain(|a| *a != DawnAbility::Brawl);
        } else {
            self.caste_abilities
                .retain(|a| Some(a) != self.supernal_ability.as_ref());
        }

        if self.caste_abilities.len() != 4 {
            if self.is_martial_arts_supernal {
                return Err(eyre!("Martial Arts Supernal requires five Caste abilities, one of which must be Brawl"));
            } else {
                return Err(eyre!("Must have exactly 5 caste abilities"));
            }
        }

        if self.is_martial_arts_supernal {
            Ok(DawnTraits::MartialArtsSupernal(self.caste_abilities))
        } else {
            Ok(DawnTraits::NotMartialArtsSupernal(
                self.supernal_ability.unwrap(),
                self.caste_abilities,
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ZenithAbility {
    Athletics,
    Integrity,
    Performance,
    Lore,
    Presence,
    Resistance,
    Survival,
    War,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZenithTraits(ZenithAbility, Vec<ZenithAbility>);

impl ZenithTraits {
    pub fn builder() -> ZenithTraitsBuilder {
        ZenithTraitsBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ZenithTraitsBuilder {
    supernal_ability: Option<ZenithAbility>,
    caste_abilities: Vec<ZenithAbility>,
}

impl ZenithTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: ZenithAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: ZenithAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<ZenithTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(ZenithTraits(ability, self.caste_abilities))
            }
        } else {
            Err(eyre!("Must specify a supernal ability"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TwilightAbility {
    Bureaucracy,
    Craft,
    Integrity,
    Investigation,
    Linguistics,
    Lore,
    Medicine,
    Occult,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TwilightTraits(TwilightAbility, Vec<TwilightAbility>);

impl TwilightTraits {
    pub fn builder() -> TwilightTraitsBuilder {
        TwilightTraitsBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct TwilightTraitsBuilder {
    supernal_ability: Option<TwilightAbility>,
    caste_abilities: Vec<TwilightAbility>,
}

impl TwilightTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: TwilightAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: TwilightAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<TwilightTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(TwilightTraits(ability, self.caste_abilities))
            }
        } else {
            Err(eyre!("Must specify a supernal ability"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum NightAbility {
    Athletics,
    Awareness,
    Dodge,
    Investigation,
    Larceny,
    Ride,
    Stealth,
    Socialize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NightTraits(NightAbility, Vec<NightAbility>);

impl NightTraits {
    pub fn builder() -> NightTraitsBuilder {
        NightTraitsBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct NightTraitsBuilder {
    supernal_ability: Option<NightAbility>,
    caste_abilities: Vec<NightAbility>,
}

impl NightTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: NightAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: NightAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<NightTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(NightTraits(ability, self.caste_abilities))
            }
        } else {
            Err(eyre!("Must specify a supernal ability"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum EclipseAbility {
    Bureaucracy,
    Larceny,
    Linguistics,
    Occult,
    Presence,
    Ride,
    Sail,
    Socialize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EclipseTraits(EclipseAbility, Vec<EclipseAbility>);

impl EclipseTraits {
    pub fn builder() -> EclipseTraitsBuilder {
        EclipseTraitsBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct EclipseTraitsBuilder {
    supernal_ability: Option<EclipseAbility>,
    caste_abilities: Vec<EclipseAbility>,
}

impl EclipseTraitsBuilder {
    pub fn with_supernal_ability(mut self, ability: EclipseAbility) -> Self {
        self.supernal_ability = Some(ability);
        self
    }

    pub fn with_caste_ability(mut self, ability: EclipseAbility) -> Self {
        self.caste_abilities.push(ability);
        self
    }

    pub fn build(mut self) -> Result<EclipseTraits> {
        if let Some(ability) = self.supernal_ability {
            self.caste_abilities.sort();
            self.caste_abilities.dedup();
            self.caste_abilities.retain(|a| *a != ability);
            if self.caste_abilities.len() != 4 {
                Err(eyre!("Must have exactly 5 caste abilities"))
            } else {
                Ok(EclipseTraits(ability, self.caste_abilities))
            }
        } else {
            Err(eyre!("Must specify a supernal ability"))
        }
    }
}
