use eyre::eyre;
use sqlx::postgres::PgHasArrayType;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EXALTTYPE", rename_all = "UPPERCASE")]
pub enum ExaltType {
    Solar,
    Lunar,
    DragonBlooded,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ATTRIBUTENAME", rename_all = "UPPERCASE")]
pub enum AttributeName {
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

impl From<AttributeName> for crate::character::traits::attributes::AttributeName {
    fn from(val: AttributeName) -> Self {
        use crate::character::traits::attributes::AttributeName as TraitsAttributeName;

        match val {
            AttributeName::Strength => TraitsAttributeName::Strength,
            AttributeName::Dexterity => TraitsAttributeName::Dexterity,
            AttributeName::Stamina => TraitsAttributeName::Stamina,
            AttributeName::Charisma => TraitsAttributeName::Charisma,
            AttributeName::Manipulation => TraitsAttributeName::Manipulation,
            AttributeName::Appearance => TraitsAttributeName::Appearance,
            AttributeName::Perception => TraitsAttributeName::Perception,
            AttributeName::Intelligence => TraitsAttributeName::Intelligence,
            AttributeName::Wits => TraitsAttributeName::Wits,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ABILITYNAME", rename_all = "UPPERCASE")]
pub enum AbilityName {
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

impl TryInto<crate::character::traits::abilities::AbilityNameNoFocus> for AbilityName {
    type Error = eyre::Report;

    fn try_into(
        self,
    ) -> Result<crate::character::traits::abilities::AbilityNameNoFocus, Self::Error> {
        use crate::character::traits::abilities::AbilityNameNoFocus;

        match self {
            Self::Archery => Ok(AbilityNameNoFocus::Archery),
            Self::Athletics => Ok(AbilityNameNoFocus::Athletics),
            Self::Awareness => Ok(AbilityNameNoFocus::Awareness),
            Self::Brawl => Ok(AbilityNameNoFocus::Brawl),
            Self::Bureaucracy => Ok(AbilityNameNoFocus::Bureaucracy),
            Self::Dodge => Ok(AbilityNameNoFocus::Dodge),
            Self::Integrity => Ok(AbilityNameNoFocus::Integrity),
            Self::Investigation => Ok(AbilityNameNoFocus::Investigation),
            Self::Larceny => Ok(AbilityNameNoFocus::Larceny),
            Self::Linguistics => Ok(AbilityNameNoFocus::Linguistics),
            Self::Lore => Ok(AbilityNameNoFocus::Lore),
            Self::Medicine => Ok(AbilityNameNoFocus::Medicine),
            Self::Melee => Ok(AbilityNameNoFocus::Melee),
            Self::Occult => Ok(AbilityNameNoFocus::Occult),
            Self::Performance => Ok(AbilityNameNoFocus::Performance),
            Self::Presence => Ok(AbilityNameNoFocus::Presence),
            Self::Resistance => Ok(AbilityNameNoFocus::Resistance),
            Self::Ride => Ok(AbilityNameNoFocus::Ride),
            Self::Sail => Ok(AbilityNameNoFocus::Sail),
            Self::Socialize => Ok(AbilityNameNoFocus::Socialize),
            Self::Stealth => Ok(AbilityNameNoFocus::Stealth),
            Self::Survival => Ok(AbilityNameNoFocus::Survival),
            Self::Thrown => Ok(AbilityNameNoFocus::Thrown),
            Self::War => Ok(AbilityNameNoFocus::War),
            Self::Craft => Err(eyre!("craft requires a focus")),
            Self::MartialArts => Err(eyre!("martial arts requires a style")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "INTIMACYTYPE", rename_all = "UPPERCASE")]
pub enum IntimacyType {
    Tie,
    Principle,
}

impl From<IntimacyType> for crate::character::traits::intimacies::IntimacyType {
    fn from(intimacy_type: IntimacyType) -> Self {
        match intimacy_type {
            IntimacyType::Tie => crate::character::traits::intimacies::IntimacyType::Tie,
            IntimacyType::Principle => {
                crate::character::traits::intimacies::IntimacyType::Principle
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "INTIMACYLEVEL", rename_all = "UPPERCASE")]
pub enum IntimacyLevel {
    Minor,
    Major,
    Defining,
}

impl From<IntimacyLevel> for crate::character::traits::intimacies::IntimacyLevel {
    fn from(intimacy_level: IntimacyLevel) -> Self {
        match intimacy_level {
            IntimacyLevel::Minor => crate::character::traits::intimacies::IntimacyLevel::Minor,
            IntimacyLevel::Major => crate::character::traits::intimacies::IntimacyLevel::Major,
            IntimacyLevel::Defining => {
                crate::character::traits::intimacies::IntimacyLevel::Defining
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WOUNDPENALTY", rename_all = "UPPERCASE")]
pub enum WoundPenalty {
    Zero,
    MinusOne,
    MinusTwo,
    MinusFour,
    Incapacitated,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "DAMAGETYPE", rename_all = "UPPERCASE")]
pub enum DamageType {
    Bashing,
    Lethal,
    Aggravated,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAGTYPE", rename_all = "UPPERCASE")]
pub enum WeaponTagType {
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
pub enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ARMORTAG", rename_all = "UPPERCASE")]
pub enum ArmorTag {
    Artifact,
    Concealable,
    Heavy,
    Light,
    Medium,
    Silent,
    Special,
}

impl PgHasArrayType for ArmorTag {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ARMORTAG")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EQUIPHAND", rename_all = "UPPERCASE")]
pub enum EquipHand {
    Main,
    Off,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMKEYWORD", rename_all = "UPPERCASE")]
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
pub enum CharmDurationType {
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
pub enum CharmActionType {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMCOSTTYPE", rename_all = "UPPERCASE")]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "PREREQUISITETYPE", rename_all = "UPPERCASE")]
pub enum PrerequisiteType {
    Ability,
    Attribute,
    Essence,
    Charm,
    ExaltType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "PREREQUISITEEXALTTYPE", rename_all = "UPPERCASE")]
pub enum PrerequisiteExaltType {
    Solar,
    Lunar,
    DragonBlooded,
    Spirit,
    SpiritOrEclipse,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "MERITTYPE", rename_all = "UPPERCASE")]
pub enum MeritType {
    Innate,
    Supernatural,
    Story,
    Purchased,
}
