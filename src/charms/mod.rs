use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityNameNoSubskill, attributes::AttributeName, data_source::DataSource, id::Id,
};

pub mod tables;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharmKeyword {
    Air,
    Aggravated,
    Archetype,
    Aura,
    Balanced,
    Bridge,
    Clash,
    Counterattack,
    DecisiveOnly,
    Dual,
    Excellency,
    Fire,
    Earth,
    Mute,
    Pilot,
    Protean,
    Psyche,
    Perilous,
    Ritual,
    Salient,
    Signature,
    Stackable,
    Uniform,
    Water,
    WitheringOnly,
    Wood,
    WrittenOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharmActionType {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharmCostType {
    Motes,
    Willpower,
    BashingHealth,
    LethalHealth,
    AggravatedHealth,
    AnimaLevels,
    Initiative,
    Experience,
    SilverCraftExperience,
    GoldCraftExperience,
    WhiteCraftExperience,
    SorcerousMotes,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CharmTraits {
    id: Id,
    data_source: DataSource,
    name: String,
    summary: Option<String>,
    duration: String,
    keywords: Vec<CharmKeyword>,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCharm {
    action_type: CharmActionType,
    ability: AbilityNameNoSubskill,
    ability_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
}

pub struct _LunarCharm {
    /// If None, implies Universal Charm
    action_type: CharmActionType,
    attribute: Option<AttributeName>,
    attribute_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
}

pub struct _DragonBloodedCharm {
    action_type: CharmActionType,
    ability: AbilityNameNoSubskill,
    ability_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MartialArtsCharm {
    action_type: CharmActionType,
    style: String,
    martial_arts_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    traits: CharmTraits,
}
