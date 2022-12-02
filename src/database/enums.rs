use sqlx::postgres::PgHasArrayType;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "EXALTTYPE", rename_all = "UPPERCASE")]
pub enum ExaltType {
    Solar,
    Lunar,
    DragonBlooded,
}

#[derive(Debug, sqlx::Type)]
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

#[derive(Debug, sqlx::Type)]
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "INTIMACYTYPE", rename_all = "UPPERCASE")]
pub enum IntimacyType {
    Tie,
    Principle,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "INTIMACYLEVEL", rename_all = "UPPERCASE")]
pub enum IntimacyLevel {
    Minor,
    Major,
    Defining,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "WOUNDPENALTY", rename_all = "UPPERCASE")]
pub enum WoundPenalty {
    Zero,
    MinusOne,
    MinusTwo,
    MinusFour,
    Incapacitated,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "DAMAGETYPE", rename_all = "UPPERCASE")]
pub enum DamageType {
    Bashing,
    Lethal,
    Aggravated,
}

#[derive(Debug, sqlx::Type)]
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "RANGEBAND", rename_all = "UPPERCASE")]
pub enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}


#[derive(Debug, sqlx::Type)]
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "EQUIPHAND", rename_all = "UPPERCASE")]
pub enum EquipHand {
    Main,
    Off,
}

#[derive(Debug, sqlx::Type)]
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

#[derive(Debug, sqlx::Type)]
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "CHARMACTIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmActionType {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

#[derive(Debug, sqlx::Type)]
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