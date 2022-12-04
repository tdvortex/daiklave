use sqlx::postgres::PgHasArrayType;

use crate::character::traits::abilities::AbilityNameNoFocus;
use crate::character::traits::armor::ArmorTag;
use crate::character::traits::attributes::AttributeName;
use crate::character::traits::intimacies::{IntimacyType, IntimacyLevel};
use crate::character::traits::prerequisite::ExaltTypePrerequisite;
use crate::character::traits::range_bands::RangeBand;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EXALTTYPE", rename_all = "UPPERCASE")]
pub enum ExaltTypePostgres {
    Solar,
    Lunar,
    DragonBlooded,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ATTRIBUTENAME", rename_all = "UPPERCASE")]
pub enum AttributeNamePostgres {
    Strength,
    Dexterity,
    Stamina,
    Charisma,
    Manipulation,
    Appearance,
    Perception,
    Intelligence,
    Wits,
}

impl From<AttributeNamePostgres> for AttributeName {
    fn from(val: AttributeNamePostgres) -> Self {
        match val {
            AttributeNamePostgres::Strength => Self::Strength,
            AttributeNamePostgres::Dexterity => Self::Dexterity,
            AttributeNamePostgres::Stamina => Self::Stamina,
            AttributeNamePostgres::Charisma => Self::Charisma,
            AttributeNamePostgres::Manipulation => Self::Manipulation,
            AttributeNamePostgres::Appearance => Self::Appearance,
            AttributeNamePostgres::Perception => Self::Perception,
            AttributeNamePostgres::Intelligence => Self::Intelligence,
            AttributeNamePostgres::Wits => Self::Wits,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ABILITYNAME", rename_all = "UPPERCASE")]
pub enum AbilityNamePostgres {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Craft,
    Dodge,
    Integrity,
    Investigation,
    Larceny,
    Linguistics,
    Lore,
    MartialArts,
    Medicine,
    Melee,
    Occult,
    Performance,
    Presence,
    Resistance,
    Ride,
    Sail,
    Socialize,
    Stealth,
    Survival,
    Thrown,
    War,
}

impl From<AbilityNamePostgres> for AbilityNameNoFocus {
    fn from(ability_name_postgres: AbilityNamePostgres) -> Self {
        match ability_name_postgres {
            AbilityNamePostgres::Archery => Self::Archery,
            AbilityNamePostgres::Athletics => Self::Athletics,
            AbilityNamePostgres::Awareness => Self::Awareness,
            AbilityNamePostgres::Brawl => Self::Brawl,
            AbilityNamePostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNamePostgres::Craft => Self::Craft,
            AbilityNamePostgres::Dodge => Self::Dodge,
            AbilityNamePostgres::Integrity => Self::Integrity,
            AbilityNamePostgres::Investigation => Self::Investigation,
            AbilityNamePostgres::Larceny => Self::Larceny,
            AbilityNamePostgres::Linguistics => Self::Linguistics,
            AbilityNamePostgres::Lore => Self::Lore,
            AbilityNamePostgres::MartialArts => Self::MartialArts,
            AbilityNamePostgres::Medicine => Self::Medicine,
            AbilityNamePostgres::Melee => Self::Melee,
            AbilityNamePostgres::Occult => Self::Occult,
            AbilityNamePostgres::Performance => Self::Performance,
            AbilityNamePostgres::Presence => Self::Presence,
            AbilityNamePostgres::Resistance => Self::Resistance,
            AbilityNamePostgres::Ride => Self::Ride,
            AbilityNamePostgres::Sail => Self::Sail,
            AbilityNamePostgres::Socialize => Self::Socialize,
            AbilityNamePostgres::Stealth => Self::Stealth,
            AbilityNamePostgres::Survival => Self::Survival,
            AbilityNamePostgres::Thrown => Self::Thrown,
            AbilityNamePostgres::War => Self::War,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "INTIMACYTYPE", rename_all = "UPPERCASE")]
pub enum IntimacyTypePostgres {
    Tie,
    Principle,
}

impl From<IntimacyTypePostgres> for IntimacyType {
    fn from(intimacy_type_postgres: IntimacyTypePostgres) -> Self {
        match intimacy_type_postgres {
            IntimacyTypePostgres::Tie => IntimacyType::Tie,
            IntimacyTypePostgres::Principle => IntimacyType::Principle,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "INTIMACYLEVEL", rename_all = "UPPERCASE")]
pub enum IntimacyLevelPostgres {
    Minor,
    Major,
    Defining,
}

impl From<IntimacyLevelPostgres> for IntimacyLevel {
    fn from(intimacy_level: IntimacyLevelPostgres) -> Self {
        match intimacy_level {
            IntimacyLevelPostgres::Minor => IntimacyLevel::Minor,
            IntimacyLevelPostgres::Major => IntimacyLevel::Major,
            IntimacyLevelPostgres::Defining => IntimacyLevel::Defining,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WOUNDPENALTY", rename_all = "UPPERCASE")]
pub enum WoundPenaltyPostgres {
    Zero,
    MinusOne,
    MinusTwo,
    MinusFour,
    Incapacitated,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "DAMAGETYPE", rename_all = "UPPERCASE")]
pub enum DamageTypePostgres {
    Bashing,
    Lethal,
    Aggravated,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAGTYPE", rename_all = "UPPERCASE")]
pub enum WeaponTagTypePostgres {
    Archery,
    Artifact,
    Balanced,
    Bashing,
    Brawl,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Exceptional,
    Flame,
    Flexible,
    Grappling,
    Heavy,
    Improvised,
    Lethal,
    Light,
    MartialArts,
    Medium,
    Melee,
    Mounted,
    OneHanded,
    Natural,
    Piercing,
    Poisonable,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Special,
    Subtle,
    Thrown,
    TwoHanded,
    Worn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "RANGEBAND", rename_all = "UPPERCASE")]
pub enum RangeBandPostgres {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}

impl From<RangeBandPostgres> for RangeBand {
    fn from(range: RangeBandPostgres) -> Self {
        match range {
            RangeBandPostgres::Close => Self::Close,
            RangeBandPostgres::Short => Self::Short,
            RangeBandPostgres::Medium => Self::Medium,
            RangeBandPostgres::Long => Self::Long,
            RangeBandPostgres::Extreme => Self::Extreme,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ARMORTAG", rename_all = "UPPERCASE")]
pub enum ArmorTagPostgres {
    Artifact,
    Concealable,
    Heavy,
    Light,
    Medium,
    Silent,
    Special,
}

impl PgHasArrayType for ArmorTagPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ARMORTAG")
    }
}

impl From<ArmorTagPostgres> for ArmorTag {
    fn from(tag: ArmorTagPostgres) -> Self {
        match tag {
            ArmorTagPostgres::Artifact => Self::Artifact,
            ArmorTagPostgres::Concealable => Self::Concealable,
            ArmorTagPostgres::Heavy => Self::Heavy,
            ArmorTagPostgres::Light => Self::Light,
            ArmorTagPostgres::Medium => Self::Medium,
            ArmorTagPostgres::Silent => Self::Silent,
            ArmorTagPostgres::Special => Self::Special,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EQUIPHAND", rename_all = "UPPERCASE")]
pub enum EquipHandPostgres {
    Main,
    Off,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMKEYWORD", rename_all = "UPPERCASE")]
pub enum CharmKeywordPostgres {
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
    Salient,
    Signature,
    Stackable,
    Uniform,
    Water,
    WitheringOnly,
    Wood,
    WrittenOnly,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMDURATIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmDurationTypePostgres {
    Instant,
    Tick,
    Turn,
    Round,
    Scene,
    Indefinite,
    Permanent,
    Special,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMACTIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmActionTypePostgres {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMCOSTTYPE", rename_all = "UPPERCASE")]
pub enum CharmCostTypePostgres {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "PREREQUISITETYPE", rename_all = "UPPERCASE")]
pub enum PrerequisiteTypePostgres {
    Ability,
    Attribute,
    Essence,
    Charm,
    ExaltType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "PREREQUISITEEXALTTYPE", rename_all = "UPPERCASE")]
pub enum PrerequisiteExaltTypePostgres {
    Solar,
    Lunar,
    DragonBlooded,
    Spirit,
    SpiritOrEclipse,
}

impl From<PrerequisiteExaltTypePostgres> for ExaltTypePrerequisite {
    fn from(exalt_type: PrerequisiteExaltTypePostgres) -> Self {
        match exalt_type {
            PrerequisiteExaltTypePostgres::Solar => Self::Solar,
            PrerequisiteExaltTypePostgres::Lunar => Self::Lunar,
            PrerequisiteExaltTypePostgres::DragonBlooded => Self::DragonBlooded,
            PrerequisiteExaltTypePostgres::Spirit => Self::Spirit,
            PrerequisiteExaltTypePostgres::SpiritOrEclipse => Self::SpiritOrEclipse,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "MERITTYPE", rename_all = "UPPERCASE")]
pub enum MeritTypePostgres {
    Innate,
    Supernatural,
    Story,
    Purchased,
}
