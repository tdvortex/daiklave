use daiklave_core::{Character, player::Player, id::Id, attributes::AttributeName, abilities::{AbilityNameNoSubskill, AbilityNameVanilla}, intimacies::{IntimacyType, IntimacyLevel}, health::{WoundPenalty, DamageLevel}, weapons::{RangeBand, WeaponTag}, armor::ArmorTag, merits::{MeritType, MeritTemplate}, prerequisite::ExaltTypePrerequisite, charms::{CharmCostType, CharmKeyword, CharmActionType}, character::{CharacterBuilder, Willpower, ExperiencePoints}};
use sqlx::{PgPool, query, postgres::PgHasArrayType, Transaction, Postgres};
use eyre::{eyre, WrapErr, Result, Report};

pub async fn destroy_character(pool: &PgPool, id: i32) -> Result<()> {
    query!(
        "DELETE FROM characters
        WHERE id = $1",
        id as i32
    )
    .execute(pool)
    .await
    .wrap_err_with(|| format!("Database error deleting character {}", id))?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EXALTTYPE", rename_all = "UPPERCASE")]
pub enum ExaltTypePostgres {
    Solar,
    Lunar,
    DragonBlooded,
}

#[derive(Debug)]
pub struct CharacterRow {
    pub id: i32,
    pub player_id: i32,
    pub campaign_id: Option<i32>,
    pub name: String,
    pub concept: Option<String>,
    pub exalt_type: Option<ExaltTypePostgres>,
    pub current_willpower: i16,
    pub max_willpower: i16,
    pub current_experience: i16,
    pub total_experience: i16,
}

impl sqlx::Type<sqlx::Postgres> for CharacterRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("characters")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CharacterRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let player_id = decoder.try_decode::<i32>()?;
        let campaign_id = decoder.try_decode::<Option<i32>>()?;
        let name = decoder.try_decode::<String>()?;
        let concept = decoder.try_decode::<Option<String>>()?;
        let exalt_type = decoder.try_decode::<Option<ExaltTypePostgres>>()?;
        let current_willpower = decoder.try_decode::<i16>()?;
        let max_willpower = decoder.try_decode::<i16>()?;
        let current_experience = decoder.try_decode::<i16>()?;
        let total_experience = decoder.try_decode::<i16>()?;

        Ok(Self {
            id,
            player_id,
            campaign_id,
            name,
            concept,
            exalt_type,
            current_willpower,
            max_willpower,
            current_experience,
            total_experience,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "players")]
pub struct PlayerRow {
    pub id: i32,
    pub name: String,
}

impl From<PlayerRow> for Player {
    fn from(row: PlayerRow) -> Self {
        Player {
            id: Id::Database(row.id),
            name: row.name,
        }
    }
}

#[derive(Debug)]
pub struct CampaignRow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub bot_channel: i64,
}

impl sqlx::Type<sqlx::Postgres> for CampaignRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("campaigns")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CampaignRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let description = decoder.try_decode::<Option<String>>()?;
        let bot_channel = decoder.try_decode::<i64>()?;

        Ok(Self {
            id,
            name,
            description,
            bot_channel,
        })
    }
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
    fn from(value: AttributeNamePostgres) -> Self {
        match value {
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

impl From<AttributeName> for AttributeNamePostgres {
    fn from(value: AttributeName) -> Self {
        match value {
            AttributeName::Strength => Self::Strength,
            AttributeName::Dexterity => Self::Dexterity,
            AttributeName::Stamina => Self::Stamina,
            AttributeName::Charisma => Self::Charisma,
            AttributeName::Manipulation => Self::Manipulation,
            AttributeName::Appearance => Self::Appearance,
            AttributeName::Perception => Self::Perception,
            AttributeName::Intelligence => Self::Intelligence,
            AttributeName::Wits => Self::Wits,
        }
    }
}

impl PgHasArrayType for AttributeNamePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ATTRIBUTENAME")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "attributes")]
pub struct AttributeRow {
    pub character_id: i32,
    pub name: AttributeNamePostgres,
    pub dots: i16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ABILITYNAMEVANILLA", rename_all = "UPPERCASE")]
pub enum AbilityNameVanillaPostgres {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Dodge,
    Integrity,
    Investigation,
    Larceny,
    Linguistics,
    Lore,
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

impl PgHasArrayType for AbilityNameVanillaPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ABILITYNAMEVANILLA")
    }
}

impl From<AbilityNameVanillaPostgres> for AbilityNameNoSubskill {
    fn from(ability_name_postgres: AbilityNameVanillaPostgres) -> Self {
        match ability_name_postgres {
            AbilityNameVanillaPostgres::Archery => Self::Archery,
            AbilityNameVanillaPostgres::Athletics => Self::Athletics,
            AbilityNameVanillaPostgres::Awareness => Self::Awareness,
            AbilityNameVanillaPostgres::Brawl => Self::Brawl,
            AbilityNameVanillaPostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanillaPostgres::Dodge => Self::Dodge,
            AbilityNameVanillaPostgres::Integrity => Self::Integrity,
            AbilityNameVanillaPostgres::Investigation => Self::Investigation,
            AbilityNameVanillaPostgres::Larceny => Self::Larceny,
            AbilityNameVanillaPostgres::Linguistics => Self::Linguistics,
            AbilityNameVanillaPostgres::Lore => Self::Lore,
            AbilityNameVanillaPostgres::Medicine => Self::Medicine,
            AbilityNameVanillaPostgres::Melee => Self::Melee,
            AbilityNameVanillaPostgres::Occult => Self::Occult,
            AbilityNameVanillaPostgres::Performance => Self::Performance,
            AbilityNameVanillaPostgres::Presence => Self::Presence,
            AbilityNameVanillaPostgres::Resistance => Self::Resistance,
            AbilityNameVanillaPostgres::Ride => Self::Ride,
            AbilityNameVanillaPostgres::Sail => Self::Sail,
            AbilityNameVanillaPostgres::Socialize => Self::Socialize,
            AbilityNameVanillaPostgres::Stealth => Self::Stealth,
            AbilityNameVanillaPostgres::Survival => Self::Survival,
            AbilityNameVanillaPostgres::Thrown => Self::Thrown,
            AbilityNameVanillaPostgres::War => Self::War,
        }
    }
}

impl From<AbilityNameVanillaPostgres> for AbilityNameVanilla {
    fn from(ability_name_postgres: AbilityNameVanillaPostgres) -> Self {
        match ability_name_postgres {
            AbilityNameVanillaPostgres::Archery => Self::Archery,
            AbilityNameVanillaPostgres::Athletics => Self::Athletics,
            AbilityNameVanillaPostgres::Awareness => Self::Awareness,
            AbilityNameVanillaPostgres::Brawl => Self::Brawl,
            AbilityNameVanillaPostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanillaPostgres::Dodge => Self::Dodge,
            AbilityNameVanillaPostgres::Integrity => Self::Integrity,
            AbilityNameVanillaPostgres::Investigation => Self::Investigation,
            AbilityNameVanillaPostgres::Larceny => Self::Larceny,
            AbilityNameVanillaPostgres::Linguistics => Self::Linguistics,
            AbilityNameVanillaPostgres::Lore => Self::Lore,
            AbilityNameVanillaPostgres::Medicine => Self::Medicine,
            AbilityNameVanillaPostgres::Melee => Self::Melee,
            AbilityNameVanillaPostgres::Occult => Self::Occult,
            AbilityNameVanillaPostgres::Performance => Self::Performance,
            AbilityNameVanillaPostgres::Presence => Self::Presence,
            AbilityNameVanillaPostgres::Resistance => Self::Resistance,
            AbilityNameVanillaPostgres::Ride => Self::Ride,
            AbilityNameVanillaPostgres::Sail => Self::Sail,
            AbilityNameVanillaPostgres::Socialize => Self::Socialize,
            AbilityNameVanillaPostgres::Stealth => Self::Stealth,
            AbilityNameVanillaPostgres::Survival => Self::Survival,
            AbilityNameVanillaPostgres::Thrown => Self::Thrown,
            AbilityNameVanillaPostgres::War => Self::War,
        }
    }
}

impl From<AbilityNameVanilla> for AbilityNameVanillaPostgres {
    fn from(ability_name: AbilityNameVanilla) -> Self {
        match ability_name {
            AbilityNameVanilla::Archery => Self::Archery,
            AbilityNameVanilla::Athletics => Self::Athletics,
            AbilityNameVanilla::Awareness => Self::Awareness,
            AbilityNameVanilla::Brawl => Self::Brawl,
            AbilityNameVanilla::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanilla::Dodge => Self::Dodge,
            AbilityNameVanilla::Integrity => Self::Integrity,
            AbilityNameVanilla::Investigation => Self::Investigation,
            AbilityNameVanilla::Larceny => Self::Larceny,
            AbilityNameVanilla::Linguistics => Self::Linguistics,
            AbilityNameVanilla::Lore => Self::Lore,
            AbilityNameVanilla::Medicine => Self::Medicine,
            AbilityNameVanilla::Melee => Self::Melee,
            AbilityNameVanilla::Occult => Self::Occult,
            AbilityNameVanilla::Performance => Self::Performance,
            AbilityNameVanilla::Presence => Self::Presence,
            AbilityNameVanilla::Resistance => Self::Resistance,
            AbilityNameVanilla::Ride => Self::Ride,
            AbilityNameVanilla::Sail => Self::Sail,
            AbilityNameVanilla::Socialize => Self::Socialize,
            AbilityNameVanilla::Stealth => Self::Stealth,
            AbilityNameVanilla::Survival => Self::Survival,
            AbilityNameVanilla::Thrown => Self::Thrown,
            AbilityNameVanilla::War => Self::War,
        }
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "abilities")]
pub struct AbilityRow {
    pub character_id: i32,
    pub name: AbilityNameVanillaPostgres,
    pub dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "specialties")]
pub struct SpecialtyRow {
    pub character_id: i32,
    pub name: AbilityNameVanillaPostgres,
    pub specialty: String,
}

impl PgHasArrayType for SpecialtyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_specialties")
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
            IntimacyTypePostgres::Tie => Self::Tie,
            IntimacyTypePostgres::Principle => Self::Principle,
        }
    }
}

impl From<IntimacyType> for IntimacyTypePostgres {
    fn from(intimacy_type: IntimacyType) -> Self {
        match intimacy_type {
            IntimacyType::Tie => Self::Tie,
            IntimacyType::Principle => Self::Principle,
        }
    }
}

impl PgHasArrayType for IntimacyTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_INTIMACYTYPE")
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
    fn from(intimacy_level_postgres: IntimacyLevelPostgres) -> Self {
        match intimacy_level_postgres {
            IntimacyLevelPostgres::Minor => Self::Minor,
            IntimacyLevelPostgres::Major => Self::Major,
            IntimacyLevelPostgres::Defining => Self::Defining,
        }
    }
}

impl From<IntimacyLevel> for IntimacyLevelPostgres {
    fn from(intimacy_level: IntimacyLevel) -> Self {
        match intimacy_level {
            IntimacyLevel::Minor => Self::Minor,
            IntimacyLevel::Major => Self::Major,
            IntimacyLevel::Defining => Self::Defining,
        }
    }
}

impl PgHasArrayType for IntimacyLevelPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_INTIMACYLEVEL")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "intimacies")]
pub struct IntimacyRow {
    pub id: i32,
    pub character_id: i32,
    pub intimacy_type: IntimacyTypePostgres,
    pub level: IntimacyLevelPostgres,
    pub description: String,
}

impl PgHasArrayType for IntimacyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_intimacies")
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

impl From<WoundPenalty> for WoundPenaltyPostgres {
    fn from(value: WoundPenalty) -> Self {
        match value {
            WoundPenalty::Zero => Self::Zero,
            WoundPenalty::MinusOne => Self::MinusOne,
            WoundPenalty::MinusTwo => Self::MinusTwo,
            WoundPenalty::MinusFour => Self::MinusFour,
            WoundPenalty::Incapacitated => Self::Incapacitated,
        }
    }
}

impl From<WoundPenaltyPostgres> for WoundPenalty {
    fn from(value: WoundPenaltyPostgres) -> Self {
        match value {
            WoundPenaltyPostgres::Zero => Self::Zero,
            WoundPenaltyPostgres::MinusOne => Self::MinusOne,
            WoundPenaltyPostgres::MinusTwo => Self::MinusTwo,
            WoundPenaltyPostgres::MinusFour => Self::MinusFour,
            WoundPenaltyPostgres::Incapacitated => Self::Incapacitated,
        }
    }
}

impl PgHasArrayType for WoundPenaltyPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_WOUNDPENALTY")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "DAMAGETYPE", rename_all = "UPPERCASE")]
pub enum DamageTypePostgres {
    None,
    Bashing,
    Lethal,
    Aggravated,
}


impl From<DamageLevel> for DamageTypePostgres {
    fn from(value: DamageLevel) -> Self {
        match value {
            DamageLevel::None => DamageTypePostgres::None,
            DamageLevel::Bashing => DamageTypePostgres::Bashing,
            DamageLevel::Lethal => DamageTypePostgres::Lethal,
            DamageLevel::Aggravated => DamageTypePostgres::Aggravated,
        }
    }
}

impl PgHasArrayType for DamageTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_DAMAGETYPE")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "health_boxes")]
pub struct HealthBoxRow {
    pub character_id: i32,
    pub position: i16,
    pub wound_penalty: WoundPenaltyPostgres,
    pub damage: Option<DamageTypePostgres>,
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

impl PgHasArrayType for RangeBandPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_RANGEBAND")
    }
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

impl From<RangeBand> for RangeBandPostgres {
    fn from(range: RangeBand) -> Self {
        match range {
            RangeBand::Close => Self::Close,
            RangeBand::Short => Self::Short,
            RangeBand::Medium => Self::Medium,
            RangeBand::Long => Self::Long,
            RangeBand::Extreme => Self::Extreme,
        }
    }
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

impl PgHasArrayType for WeaponTagTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_WEAPONTAGTYPE")
    }
}

impl From<WeaponTag> for WeaponTagTypePostgres {
    fn from(tag: WeaponTag) -> Self {
        match tag {
            WeaponTag::Archery(_) => Self::Archery,
            WeaponTag::Artifact => Self::Artifact,
            WeaponTag::Balanced => Self::Balanced,
            WeaponTag::Bashing => Self::Bashing,
            WeaponTag::Brawl => Self::Brawl,
            WeaponTag::Chopping => Self::Chopping,
            WeaponTag::Concealable => Self::Concealable,
            WeaponTag::Crossbow => Self::Crossbow,
            WeaponTag::Cutting => Self::Cutting,
            WeaponTag::Disarming => Self::Disarming,
            WeaponTag::Exceptional => Self::Exceptional,
            WeaponTag::Flame => Self::Flame,
            WeaponTag::Flexible => Self::Flexible,
            WeaponTag::Grappling => Self::Grappling,
            WeaponTag::Heavy => Self::Heavy,
            WeaponTag::Improvised => Self::Improvised,
            WeaponTag::Lethal => Self::Lethal,
            WeaponTag::Light => Self::Light,
            WeaponTag::MartialArts(_) => Self::MartialArts,
            WeaponTag::Medium => Self::Medium,
            WeaponTag::Melee => Self::Melee,
            WeaponTag::Mounted => Self::Mounted,
            WeaponTag::OneHanded => Self::OneHanded,
            WeaponTag::Natural => Self::Natural,
            WeaponTag::Piercing => Self::Piercing,
            WeaponTag::Poisonable => Self::Poisonable,
            WeaponTag::Powerful => Self::Powerful,
            WeaponTag::Reaching => Self::Reaching,
            WeaponTag::Shield => Self::Shield,
            WeaponTag::Slow => Self::Slow,
            WeaponTag::Smashing => Self::Smashing,
            WeaponTag::Special => Self::Special,
            WeaponTag::Subtle => Self::Subtle,
            WeaponTag::Thrown(_) => Self::Thrown,
            WeaponTag::TwoHanded => Self::TwoHanded,
            WeaponTag::Worn => Self::Worn,
        }
    }
}

impl TryFrom<WeaponTagTypePostgres> for WeaponTag {
    type Error = eyre::Report;

    fn try_from(value: WeaponTagTypePostgres) -> Result<Self, Self::Error> {
        match value {
            WeaponTagTypePostgres::Archery => Err(eyre!("Range band missing for Archery tag")),
            WeaponTagTypePostgres::Thrown => Err(eyre!("Range band missing for Thrown tag")),
            WeaponTagTypePostgres::MartialArts => Err(eyre!("Style missing for Martial Arts tag")),
            WeaponTagTypePostgres::Artifact => Ok(Self::Artifact),
            WeaponTagTypePostgres::Balanced => Ok(Self::Balanced),
            WeaponTagTypePostgres::Bashing => Ok(Self::Bashing),
            WeaponTagTypePostgres::Brawl => Ok(Self::Brawl),
            WeaponTagTypePostgres::Chopping => Ok(Self::Chopping),
            WeaponTagTypePostgres::Concealable => Ok(Self::Concealable),
            WeaponTagTypePostgres::Crossbow => Ok(Self::Crossbow),
            WeaponTagTypePostgres::Cutting => Ok(Self::Cutting),
            WeaponTagTypePostgres::Disarming => Ok(Self::Disarming),
            WeaponTagTypePostgres::Exceptional => Ok(Self::Exceptional),
            WeaponTagTypePostgres::Flame => Ok(Self::Flame),
            WeaponTagTypePostgres::Flexible => Ok(Self::Flexible),
            WeaponTagTypePostgres::Grappling => Ok(Self::Grappling),
            WeaponTagTypePostgres::Heavy => Ok(Self::Heavy),
            WeaponTagTypePostgres::Improvised => Ok(Self::Improvised),
            WeaponTagTypePostgres::Lethal => Ok(Self::Lethal),
            WeaponTagTypePostgres::Light => Ok(Self::Light),
            WeaponTagTypePostgres::Medium => Ok(Self::Medium),
            WeaponTagTypePostgres::Melee => Ok(Self::Melee),
            WeaponTagTypePostgres::Mounted => Ok(Self::Mounted),
            WeaponTagTypePostgres::OneHanded => Ok(Self::OneHanded),
            WeaponTagTypePostgres::Natural => Ok(Self::Natural),
            WeaponTagTypePostgres::Piercing => Ok(Self::Piercing),
            WeaponTagTypePostgres::Poisonable => Ok(Self::Poisonable),
            WeaponTagTypePostgres::Powerful => Ok(Self::Powerful),
            WeaponTagTypePostgres::Reaching => Ok(Self::Reaching),
            WeaponTagTypePostgres::Shield => Ok(Self::Shield),
            WeaponTagTypePostgres::Slow => Ok(Self::Slow),
            WeaponTagTypePostgres::Smashing => Ok(Self::Smashing),
            WeaponTagTypePostgres::Special => Ok(Self::Special),
            WeaponTagTypePostgres::Subtle => Ok(Self::Subtle),
            WeaponTagTypePostgres::TwoHanded => Ok(Self::TwoHanded),
            WeaponTagTypePostgres::Worn => Ok(Self::Worn),
        }
    }
}

#[derive(Debug)]
pub struct WeaponTagRow {
    weapon_id: i32,
    tag_type: WeaponTagTypePostgres,
    max_range: Option<RangeBandPostgres>,
    martial_arts_style: Option<String>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponTagRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("weapon_tags")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponTagRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let weapon_id = decoder.try_decode::<i32>()?;
        let tag_type = decoder.try_decode::<WeaponTagTypePostgres>()?;
        let max_range = decoder.try_decode::<Option<RangeBandPostgres>>()?;
        let martial_arts_style = decoder.try_decode::<Option<String>>()?;

        Ok(Self {
            weapon_id,
            tag_type,
            max_range,
            martial_arts_style,
        })
    }
}

impl TryFrom<WeaponTagRow> for WeaponTag {
    type Error = Report;

    fn try_from(value: WeaponTagRow) -> Result<Self, Self::Error> {
        match value.tag_type {
            WeaponTagTypePostgres::Archery => match value.max_range {
                Some(range) => Ok(Self::Archery(range.into())),
                None => Err(eyre!("Archery must have a range band")),
            },
            WeaponTagTypePostgres::Artifact => Ok(Self::Artifact),
            WeaponTagTypePostgres::Balanced => Ok(Self::Balanced),
            WeaponTagTypePostgres::Bashing => Ok(Self::Bashing),
            WeaponTagTypePostgres::Brawl => Ok(Self::Brawl),
            WeaponTagTypePostgres::Chopping => Ok(Self::Chopping),
            WeaponTagTypePostgres::Concealable => Ok(Self::Concealable),
            WeaponTagTypePostgres::Crossbow => Ok(Self::Crossbow),
            WeaponTagTypePostgres::Cutting => Ok(Self::Cutting),
            WeaponTagTypePostgres::Disarming => Ok(Self::Disarming),
            WeaponTagTypePostgres::Exceptional => Ok(Self::Exceptional),
            WeaponTagTypePostgres::Flame => Ok(Self::Flame),
            WeaponTagTypePostgres::Flexible => Ok(Self::Flexible),
            WeaponTagTypePostgres::Grappling => Ok(Self::Grappling),
            WeaponTagTypePostgres::Heavy => Ok(Self::Heavy),
            WeaponTagTypePostgres::Improvised => Ok(Self::Improvised),
            WeaponTagTypePostgres::Lethal => Ok(Self::Lethal),
            WeaponTagTypePostgres::Light => Ok(Self::Light),
            WeaponTagTypePostgres::MartialArts => match value.martial_arts_style {
                Some(style) => Ok(Self::MartialArts(style)),
                None => Err(eyre!("Martial arts must have a style")),
            },
            WeaponTagTypePostgres::Medium => Ok(Self::Medium),
            WeaponTagTypePostgres::Melee => Ok(Self::Melee),
            WeaponTagTypePostgres::Mounted => Ok(Self::Mounted),
            WeaponTagTypePostgres::OneHanded => Ok(Self::OneHanded),
            WeaponTagTypePostgres::Natural => Ok(Self::Natural),
            WeaponTagTypePostgres::Piercing => Ok(Self::Piercing),
            WeaponTagTypePostgres::Poisonable => Ok(Self::Poisonable),
            WeaponTagTypePostgres::Powerful => Ok(Self::Powerful),
            WeaponTagTypePostgres::Reaching => Ok(Self::Reaching),
            WeaponTagTypePostgres::Shield => Ok(Self::Shield),
            WeaponTagTypePostgres::Slow => Ok(Self::Slow),
            WeaponTagTypePostgres::Smashing => Ok(Self::Smashing),
            WeaponTagTypePostgres::Special => Ok(Self::Special),
            WeaponTagTypePostgres::Subtle => Ok(Self::Subtle),
            WeaponTagTypePostgres::Thrown => match value.max_range {
                Some(range) => Ok(Self::Thrown(range.into())),
                None => Err(eyre!("Thrown must have a range band")),
            },
            WeaponTagTypePostgres::TwoHanded => Ok(Self::TwoHanded),
            WeaponTagTypePostgres::Worn => Ok(Self::Worn),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EQUIPHAND", rename_all = "UPPERCASE")]
pub enum EquipHandPostgres {
    Main,
    Off,
}

impl PgHasArrayType for EquipHandPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_EQUIPHAND")
    }
}

#[derive(Debug)]
pub struct WeaponRow {
    pub id: i32,
    pub name: String,
    pub book_title: Option<String>,
    pub page_number: Option<i16>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("weapons")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct WeaponEquippedRow {
    pub character_id: i32,
    pub weapon_id: i32,
    pub equip_hand: Option<EquipHandPostgres>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponEquippedRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("character_weapons")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponEquippedRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let character_id = decoder.try_decode::<i32>()?;
        let weapon_id = decoder.try_decode::<i32>()?;
        let equip_hand = decoder.try_decode::<Option<EquipHandPostgres>>()?;

        Ok(Self {
            character_id,
            weapon_id,
            equip_hand,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ARMORTAGTYPE", rename_all = "UPPERCASE")]
pub enum ArmorTagTypePostgres {
    Artifact,
    Concealable,
    Heavy,
    Light,
    Medium,
    Silent,
    Special,
}

impl PgHasArrayType for ArmorTagTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ARMORTAGTYPE")
    }
}

impl From<ArmorTagTypePostgres> for ArmorTag {
    fn from(tag: ArmorTagTypePostgres) -> Self {
        match tag {
            ArmorTagTypePostgres::Artifact => Self::Artifact,
            ArmorTagTypePostgres::Concealable => Self::Concealable,
            ArmorTagTypePostgres::Heavy => Self::Heavy,
            ArmorTagTypePostgres::Light => Self::Light,
            ArmorTagTypePostgres::Medium => Self::Medium,
            ArmorTagTypePostgres::Silent => Self::Silent,
            ArmorTagTypePostgres::Special => Self::Special,
        }
    }
}

impl From<ArmorTag> for ArmorTagTypePostgres {
    fn from(tag: ArmorTag) -> Self {
        match tag {
            ArmorTag::Artifact => Self::Artifact,
            ArmorTag::Concealable => Self::Concealable,
            ArmorTag::Heavy => Self::Heavy,
            ArmorTag::Light => Self::Light,
            ArmorTag::Medium => Self::Medium,
            ArmorTag::Silent => Self::Silent,
            ArmorTag::Special => Self::Special,
        }
    }
}

#[derive(Debug)]
pub struct ArmorRow {
    pub id: i32,
    pub name: String,
    pub book_title: Option<String>,
    pub page_number: Option<i16>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for ArmorRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("armor")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ArmorRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct ArmorWornRow {
    pub character_id: i32,
    pub armor_id: i32,
    pub worn: bool,
}

impl sqlx::Type<sqlx::Postgres> for ArmorWornRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("character_armor")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ArmorWornRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let character_id = decoder.try_decode::<i32>()?;
        let armor_id = decoder.try_decode::<i32>()?;
        let worn = decoder.try_decode::<bool>()?;

        Ok(Self {
            character_id,
            armor_id,
            worn,
        })
    }
}

#[derive(Debug)]
pub struct ArmorTagRow {
    pub armor_id: i32,
    pub tag_type: ArmorTagTypePostgres,
}

impl sqlx::Type<sqlx::Postgres> for ArmorTagRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("armor_tags")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ArmorTagRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let armor_id = decoder.try_decode::<i32>()?;
        let tag_type = decoder.try_decode::<ArmorTagTypePostgres>()?;

        Ok(Self { armor_id, tag_type })
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

impl PgHasArrayType for MeritTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_MERITTYPE")
    }
}

impl From<MeritTypePostgres> for MeritType {
    fn from(merit_type_postgres: MeritTypePostgres) -> Self {
        match merit_type_postgres {
            MeritTypePostgres::Innate => Self::Innate,
            MeritTypePostgres::Supernatural => Self::Supernatural,
            MeritTypePostgres::Story => Self::Story,
            MeritTypePostgres::Purchased => Self::Purchased,
        }
    }
}

impl From<MeritType> for MeritTypePostgres {
    fn from(merit_type: MeritType) -> Self {
        match merit_type {
            MeritType::Innate => Self::Innate,
            MeritType::Supernatural => Self::Supernatural,
            MeritType::Story => Self::Story,
            MeritType::Purchased => Self::Purchased,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeritTemplateRow {
    pub id: i32,
    pub name: String,
    pub requires_detail: bool,
    pub merit_type: MeritTypePostgres,
    pub description: String,
    pub book_title: Option<String>,
    pub page_number: Option<i16>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for MeritTemplateRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("merits")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MeritTemplateRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let requires_detail = decoder.try_decode::<bool>()?;
        let merit_type = decoder.try_decode::<MeritTypePostgres>()?;
        let description = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            requires_detail,
            merit_type,
            description,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct MeritTemplateInsert {
    pub name: String,
    pub merit_type: MeritTypePostgres,
    pub description: String,
    pub requires_detail: bool,
    pub book_title: Option<String>,
    pub page_number: Option<i16>,
    pub creator_id: Option<i32>,
}

impl From<MeritTemplate> for MeritTemplateInsert {
    fn from(template: MeritTemplate) -> Self {
        let nulled_creator_id = template.data_source().creator_id().and_then(|id| {
            if id.is_placeholder() {
                None
            } else {
                Some(*id)
            }
        });
        Self {
            name: template.name().to_owned(),
            merit_type: template.merit_type().into(),
            description: template.description().to_owned(),
            requires_detail: template.requires_detail(),
            book_title: template.data_source().book_title().map(|s| s.to_owned()),
            page_number: template.data_source().page_number(),
            creator_id: nulled_creator_id,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "merit_prerequisite_sets")]
pub struct MeritPrerequisiteSetRow {
    pub id: i32,
    pub merit_id: i32,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "character_merits")]
pub struct MeritDetailRow {
    pub id: i32,
    pub character_id: i32,
    pub merit_id: i32,
    pub dots: i16,
    pub detail: Option<String>,
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

impl PgHasArrayType for PrerequisiteTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_PREREQUISITETYPE")
    }
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

impl PgHasArrayType for PrerequisiteExaltTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_PREREQUISITEEXALTTYPE")
    }
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

impl From<ExaltTypePrerequisite> for PrerequisiteExaltTypePostgres {
    fn from(exalt_type: ExaltTypePrerequisite) -> Self {
        match exalt_type {
            ExaltTypePrerequisite::Solar => Self::Solar,
            ExaltTypePrerequisite::Lunar => Self::Lunar,
            ExaltTypePrerequisite::DragonBlooded => Self::DragonBlooded,
            ExaltTypePrerequisite::Spirit => Self::Spirit,
            ExaltTypePrerequisite::SpiritOrEclipse => Self::SpiritOrEclipse,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrerequisiteRow {
    pub id: i32,
    pub merit_prerequisite_set_id: Option<i32>,
    pub charm_prerequisite_set_id: Option<i32>,
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNameVanillaPostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub prerequisite_charm_id: Option<i32>,
    pub prerequisite_exalt_type: Option<PrerequisiteExaltTypePostgres>,
}


impl sqlx::Type<sqlx::Postgres> for PrerequisiteRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("prerequisites")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for PrerequisiteRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let merit_prerequisite_set_id = decoder.try_decode::<Option<i32>>()?;
        let charm_prerequisite_set_id = decoder.try_decode::<Option<i32>>()?;
        let prerequisite_type = decoder.try_decode::<PrerequisiteTypePostgres>()?;
        let ability_name = decoder.try_decode::<Option<AbilityNameVanillaPostgres>>()?;
        let subskill_name = decoder.try_decode::<Option<String>>()?;
        let attribute_name = decoder.try_decode::<Option<AttributeNamePostgres>>()?;
        let dots = decoder.try_decode::<Option<i16>>()?;
        let prerequisite_charm_id = decoder.try_decode::<Option<i32>>()?;
        let prerequisite_exalt_type =
            decoder.try_decode::<Option<PrerequisiteExaltTypePostgres>>()?;

        Ok(Self {
            id,
            merit_prerequisite_set_id,
            charm_prerequisite_set_id,
            prerequisite_type,
            ability_name,
            subskill_name,
            attribute_name,
            dots,
            prerequisite_charm_id,
            prerequisite_exalt_type,
        })
    }
}


#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "CHARMCOST")]
pub struct CharmCostPostgres {
    cost_type: CharmCostTypePostgres,
    amount: i16,
}

impl PgHasArrayType for CharmCostPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMCOST")
    }
}

impl From<CharmCostTypePostgres> for CharmCostType {
    fn from(value: CharmCostTypePostgres) -> Self {
        match value {
            CharmCostTypePostgres::Motes => Self::Motes,
            CharmCostTypePostgres::SorcerousMotes => Self::SorcerousMotes,
            CharmCostTypePostgres::Willpower => Self::Willpower,
            CharmCostTypePostgres::BashingHealth => Self::BashingHealth,
            CharmCostTypePostgres::LethalHealth => Self::LethalHealth,
            CharmCostTypePostgres::AggravatedHealth => Self::AggravatedHealth,
            CharmCostTypePostgres::AnimaLevels => Self::AnimaLevels,
            CharmCostTypePostgres::Initiative => Self::Initiative,
            CharmCostTypePostgres::Experience => Self::Experience,
            CharmCostTypePostgres::SilverCraftExperience => Self::SilverCraftExperience,
            CharmCostTypePostgres::GoldCraftExperience => Self::GoldCraftExperience,
            CharmCostTypePostgres::WhiteCraftExperience => Self::WhiteCraftExperience,
        }
    }
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

impl PgHasArrayType for CharmKeywordPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMKEYWORD")
    }
}

impl From<CharmKeywordPostgres> for CharmKeyword {
    fn from(keyword_pg: CharmKeywordPostgres) -> Self {
        match keyword_pg {
            CharmKeywordPostgres::Air => Self::Air,
            CharmKeywordPostgres::Aggravated => Self::Aggravated,
            CharmKeywordPostgres::Archetype => Self::Archetype,
            CharmKeywordPostgres::Aura => Self::Aura,
            CharmKeywordPostgres::Balanced => Self::Balanced,
            CharmKeywordPostgres::Bridge => Self::Bridge,
            CharmKeywordPostgres::Clash => Self::Clash,
            CharmKeywordPostgres::Counterattack => Self::Counterattack,
            CharmKeywordPostgres::DecisiveOnly => Self::DecisiveOnly,
            CharmKeywordPostgres::Dual => Self::Dual,
            CharmKeywordPostgres::Excellency => Self::Excellency,
            CharmKeywordPostgres::Fire => Self::Fire,
            CharmKeywordPostgres::Earth => Self::Earth,
            CharmKeywordPostgres::Mute => Self::Mute,
            CharmKeywordPostgres::Pilot => Self::Pilot,
            CharmKeywordPostgres::Protean => Self::Protean,
            CharmKeywordPostgres::Psyche => Self::Psyche,
            CharmKeywordPostgres::Perilous => Self::Perilous,
            CharmKeywordPostgres::Ritual => Self::Ritual,
            CharmKeywordPostgres::Salient => Self::Salient,
            CharmKeywordPostgres::Signature => Self::Signature,
            CharmKeywordPostgres::Stackable => Self::Stackable,
            CharmKeywordPostgres::Uniform => Self::Uniform,
            CharmKeywordPostgres::Water => Self::Water,
            CharmKeywordPostgres::WitheringOnly => Self::WitheringOnly,
            CharmKeywordPostgres::Wood => Self::Wood,
            CharmKeywordPostgres::WrittenOnly => Self::WrittenOnly,
        }
    }
}

impl From<CharmKeyword> for CharmKeywordPostgres {
    fn from(keyword: CharmKeyword) -> Self {
        match keyword {
            CharmKeyword::Air => Self::Air,
            CharmKeyword::Aggravated => Self::Aggravated,
            CharmKeyword::Archetype => Self::Archetype,
            CharmKeyword::Aura => Self::Aura,
            CharmKeyword::Balanced => Self::Balanced,
            CharmKeyword::Bridge => Self::Bridge,
            CharmKeyword::Clash => Self::Clash,
            CharmKeyword::Counterattack => Self::Counterattack,
            CharmKeyword::DecisiveOnly => Self::DecisiveOnly,
            CharmKeyword::Dual => Self::Dual,
            CharmKeyword::Excellency => Self::Excellency,
            CharmKeyword::Fire => Self::Fire,
            CharmKeyword::Earth => Self::Earth,
            CharmKeyword::Mute => Self::Mute,
            CharmKeyword::Pilot => Self::Pilot,
            CharmKeyword::Protean => Self::Protean,
            CharmKeyword::Psyche => Self::Psyche,
            CharmKeyword::Perilous => Self::Perilous,
            CharmKeyword::Ritual => Self::Ritual,
            CharmKeyword::Salient => Self::Salient,
            CharmKeyword::Signature => Self::Signature,
            CharmKeyword::Stackable => Self::Stackable,
            CharmKeyword::Uniform => Self::Uniform,
            CharmKeyword::Water => Self::Water,
            CharmKeyword::WitheringOnly => Self::WitheringOnly,
            CharmKeyword::Wood => Self::Wood,
            CharmKeyword::WrittenOnly => Self::WrittenOnly,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMACTIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmActionTypePostgres {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

impl From<CharmActionTypePostgres> for CharmActionType {
    fn from(value: CharmActionTypePostgres) -> Self {
        match value {
            CharmActionTypePostgres::Simple => Self::Simple,
            CharmActionTypePostgres::Supplemental => Self::Supplemental,
            CharmActionTypePostgres::Reflexive => Self::Reflexive,
            CharmActionTypePostgres::Permanent => Self::Permanent,
        }
    }
}

impl From<CharmActionType> for CharmActionTypePostgres {
    fn from(value: CharmActionType) -> Self {
        match value {
            CharmActionType::Simple => Self::Simple,
            CharmActionType::Supplemental => Self::Supplemental,
            CharmActionType::Reflexive => Self::Reflexive,
            CharmActionType::Permanent => Self::Permanent,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMCOSTTYPE", rename_all = "UPPERCASE")]
pub enum CharmCostTypePostgres {
    Motes,
    SorcerousMotes,
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
}

impl PgHasArrayType for CharmCostTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMCOSTTYPE")
    }
}

impl From<CharmCostType> for CharmCostTypePostgres {
    fn from(value: CharmCostType) -> Self {
        match value {
            CharmCostType::Motes => Self::Motes,
            CharmCostType::SorcerousMotes => Self::SorcerousMotes,
            CharmCostType::Willpower => Self::Willpower,
            CharmCostType::BashingHealth => Self::BashingHealth,
            CharmCostType::LethalHealth => Self::LethalHealth,
            CharmCostType::AggravatedHealth => Self::AggravatedHealth,
            CharmCostType::AnimaLevels => Self::AnimaLevels,
            CharmCostType::Initiative => Self::Initiative,
            CharmCostType::Experience => Self::Experience,
            CharmCostType::SilverCraftExperience => Self::SilverCraftExperience,
            CharmCostType::GoldCraftExperience => Self::GoldCraftExperience,
            CharmCostType::WhiteCraftExperience => Self::WhiteCraftExperience,
        }
    }
}


#[derive(Debug)]
pub(crate) struct MartialArtsStyleRow {
    id: i32,
    name: String,
    description: String,
    book_title: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for MartialArtsStyleRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("martial_arts_styles")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MartialArtsStyleRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let description = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            description,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "character_martial_arts")]
pub(crate) struct CharacterMartialArtsRow {
    character_id: i32,
    style_id: i32,
    dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "character_martial_arts_specialties")]
pub(crate) struct CharacterMartialArtsSpecialtyRow {
    character_id: i32,
    style_id: i32,
    specialty: String,
}
#[derive(Debug)]
pub(crate) struct MartialArtsCharmRow {
    id: i32,
    style_id: i32,
    ability_dots_required: i16,
    essence_dots_required: i16,
    name: String,
    summary: Option<String>,
    description: String,
    action_type: CharmActionTypePostgres,
    duration: String,
    book_title: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for MartialArtsCharmRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("martial_arts_charms")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MartialArtsCharmRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let style_id = decoder.try_decode::<i32>()?;
        let ability_dots_required = decoder.try_decode::<i16>()?;
        let essence_dots_required = decoder.try_decode::<i16>()?;
        let name = decoder.try_decode::<String>()?;
        let summary = decoder.try_decode::<Option<String>>()?;
        let description = decoder.try_decode::<String>()?;
        let action_type = decoder.try_decode::<CharmActionTypePostgres>()?;
        let duration = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            style_id,
            ability_dots_required,
            essence_dots_required,
            name,
            summary,
            description,
            action_type,
            duration,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charms_keywords")]
pub(crate) struct MartialArtsCharmKeywordRow {
    charm_id: i32,
    charm_keyword: CharmKeywordPostgres,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charms_costs")]
pub(crate) struct MartialArtsCharmCostRow {
    charm_id: i32,
    cost: CharmCostTypePostgres,
    amount: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charm_tree")]
pub(crate) struct MartialArtsCharmTreeRow {
    child_id: i32,
    parent_id: i32,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "craft_abilities")]
pub(crate) struct CraftAbilityRow {
    character_id: i32,
    focus: String,
    dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "craft_ability_specialties")]
pub(crate) struct CraftAbilitySpecialtyRow {
    character_id: i32,
    focus: String,
    specialty: String,
}


#[derive(Debug)]
struct GetCharacter {
    character: CharacterRow,
    player: PlayerRow,
    campaign: Option<CampaignRow>,
    attributes: Vec<AttributeRow>,
    abilities: Vec<AbilityRow>,
    specialties: Option<Vec<SpecialtyRow>>,
    intimacies: Option<Vec<IntimacyRow>>,
    health_boxes: Vec<HealthBoxRow>,
    weapons_owned: Option<Vec<WeaponRow>>,
    weapon_tags: Option<Vec<WeaponTagRow>>,
    weapons_equipped: Option<Vec<WeaponEquippedRow>>,
    armor_owned: Option<Vec<ArmorRow>>,
    armor_tags: Option<Vec<ArmorTagRow>>,
    armor_worn: Option<Vec<ArmorWornRow>>,
    merit_templates: Option<Vec<MeritTemplateRow>>,
    merit_details: Option<Vec<MeritDetailRow>>,
    merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    merit_prerequisites: Option<Vec<PrerequisiteRow>>,
    martial_arts_styles: Option<Vec<MartialArtsStyleRow>>,
    character_martial_arts_styles: Option<Vec<CharacterMartialArtsRow>>,
    martial_arts_specialties: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
    martial_arts_charms: Option<Vec<MartialArtsCharmRow>>,
    martial_arts_charm_keywords: Option<Vec<MartialArtsCharmKeywordRow>>,
    martial_arts_charms_costs: Option<Vec<MartialArtsCharmCostRow>>,
    martial_arts_charm_tree: Option<Vec<MartialArtsCharmTreeRow>>,
    craft_abilities: Option<Vec<CraftAbilityRow>>,
    craft_specialties: Option<Vec<CraftAbilitySpecialtyRow>>,
}

pub async fn retrieve_character(pool: &PgPool, character_id: i32) -> Result<Option<Character>> {
    let mut transaction = pool
        .begin()
        .await
        .wrap_err("Error attempting to start transaction")?;

    let maybe_character = retrieve_character_transaction(&mut transaction, character_id).await?;

    transaction
        .commit()
        .await
        .wrap_err("Error attempting to commit transaction")?;
    Ok(maybe_character)
}

async fn retrieve_character_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<Option<Character>> {
    let maybe_get_character =
        sqlx::query_file_as!(GetCharacter, "src/retrieve.sql", character_id)
            .fetch_optional(&mut *transaction)
            .await
            .wrap_err_with(|| {
                format!(
                    "Database error trying to retrieve character with id: {}",
                    character_id
                )
            })?;

    if let Some(get_character) = maybe_get_character {
        Ok(Some(get_character.try_into().wrap_err_with(|| {
            format!(
                "Error attempting to convert database output to Character for character_id {}",
                character_id
            )
        })?))
    } else {
        // Valid query with no character
        Ok(None)
    }
}

struct AllMartialArtsRows {
    pub style_rows: Option<Vec<MartialArtsStyleRow>>,
    pub character_style_rows: Option<Vec<CharacterMartialArtsRow>>,
    pub specialty_rows: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
    pub martial_arts_charm_rows: Option<Vec<MartialArtsCharmRow>>,
    pub charm_keyword_rows: Option<Vec<MartialArtsCharmKeywordRow>>,
    pub charm_cost_rows: Option<Vec<MartialArtsCharmCostRow>>,
    pub charm_tree_rows: Option<Vec<MartialArtsCharmTreeRow>>,
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        Character::builder(self.character.id, self.player.into())
            .apply_campaign_row(self.campaign)
            .apply_character_row(self.character)
            .wrap_err("Could not apply character row")?
            .apply_attribute_rows(self.attributes)
            .wrap_err("Could not apply attribute rows")?
            .apply_abilities_and_specialties_rows(self.abilities, self.specialties)
            .wrap_err("Could not apply ability and specialty rows")?
            .apply_craft(self.craft_abilities, self.craft_specialties)
            .wrap_err("Could not apply craft rows")?
            .apply_intimacy_rows(self.intimacies)
            .apply_health_box_rows(self.health_boxes)
            .apply_weapon_rows(self.weapons_owned, self.weapon_tags, self.weapons_equipped)
            .wrap_err("Could not apply weapon rows")?
            .apply_armor_rows(self.armor_owned, self.armor_tags, self.armor_worn)
            .wrap_err("Could not apply armor rows")?
            .apply_merits_rows(
                self.merit_templates,
                self.merit_details,
                self.merit_prerequisite_sets,
                self.merit_prerequisites,
            )
            .wrap_err("Could not apply merit rows")?
            .apply_martial_arts(AllMartialArtsRows {
                style_rows: self.martial_arts_styles,
                character_style_rows: self.character_martial_arts_styles,
                specialty_rows: self.martial_arts_specialties,
                martial_arts_charm_rows: self.martial_arts_charms,
                charm_keyword_rows: self.martial_arts_charm_keywords,
                charm_cost_rows: self.martial_arts_charms_costs,
                charm_tree_rows: self.martial_arts_charm_tree,
            })
            .wrap_err("Could not apply martial arts rows")?
            .build()
    }
}


pub async fn update_character(pool: &PgPool, character: &Character) -> Result<Character> {
    todo!()
}

