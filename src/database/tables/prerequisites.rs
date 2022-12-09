use crate::character::traits::prerequisite::ExaltTypePrerequisite;
use eyre::Result;
use sqlx::postgres::PgHasArrayType;

use super::{abilities::AbilityNamePostgres, attributes::AttributeNamePostgres};

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

#[derive(Debug)]
pub struct PrerequisiteRow {
    pub id: i32,
    pub merit_prerequisite_set_id: Option<i32>,
    pub charm_prerequisite_set_id: Option<i32>,
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNamePostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub prerequisite_charm_id: Option<i32>,
    pub prerequisite_exalt_type: Option<PrerequisiteExaltTypePostgres>,
}

#[derive(Debug)]
pub struct PrerequisiteInsert {
    pub merit_prerequisite_set_id: Option<i32>,
    pub charm_prerequisite_set_id: Option<i32>,
    pub prerequisite_type: PrerequisiteTypePostgres,
    pub ability_name: Option<AbilityNamePostgres>,
    pub subskill_name: Option<String>,
    pub attribute_name: Option<AttributeNamePostgres>,
    pub dots: Option<i16>,
    pub charm_id: Option<i32>,
    pub exalt_type: Option<PrerequisiteExaltTypePostgres>,
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
        let ability_name = decoder.try_decode::<Option<AbilityNamePostgres>>()?;
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
