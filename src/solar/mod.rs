use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityNameNoSubskill, essence::Essence, limit::Limit};

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