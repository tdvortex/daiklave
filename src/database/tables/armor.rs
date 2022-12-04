use std::collections::HashMap;
use eyre::{eyre, Result};

use sqlx::postgres::PgHasArrayType;

use crate::character::{traits::armor::{ArmorTag, ArmorItem}, builder::CharacterBuilder};

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

#[derive(Debug)]
pub struct ArmorRow {
    pub id: i32,
    pub name: String,
    pub tags: Vec<ArmorTagPostgres>,
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
        let tags = decoder.try_decode::<Vec<ArmorTagPostgres>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            tags,
            creator_id,
        })
    }
}

impl TryFrom<ArmorRow> for crate::character::traits::armor::ArmorItem {
    type Error = eyre::Report;

    fn try_from(value: ArmorRow) -> Result<Self, Self::Error> {
        Self::new(
            value.name,
            value.tags.into_iter().map(|tag| tag.into()).collect(),
            Some(value.id),
        )
    }
}

#[derive(Debug)]
pub struct ArmorWornRow {
    pub character_id: i32,
    pub armor_id: i32,
    pub worn: bool,
    pub creator_id: Option<i32>,
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
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            character_id,
            armor_id,
            worn,
            creator_id,
        })
    }
}

impl CharacterBuilder {
    pub fn apply_armor_rows(
        &mut self,
        armor_owned: Option<Vec<ArmorRow>>,
        armor_worn: Option<Vec<ArmorWornRow>>,
    ) -> Result<&mut Self> {
        if armor_owned.is_none() {
            if armor_worn.is_none() {
                return Ok(self);
            } else {
                return Err(eyre!("cannot wear armor that is not owned"));
            }
        }

        let mut armor_hashmap = HashMap::new();

        for armor_row in armor_owned.unwrap().into_iter() {
            let tags = armor_row.tags.into_iter().map(|tag| tag.into()).collect();
            let armor_item = ArmorItem::new(armor_row.name, tags, Some(armor_row.id))?;
            armor_hashmap.insert(armor_row.id, (armor_item, false));
        }

        if let Some(armor_worn_rows) = armor_worn {
            for armor_worn_row in armor_worn_rows.into_iter() {
                if armor_worn_row.worn {
                    let (_, worn) =
                        armor_hashmap
                            .get_mut(&armor_worn_row.armor_id)
                            .ok_or_else(|| {
                                eyre!(
                                    "cannot equip unowned armor item {}",
                                    armor_worn_row.armor_id
                                )
                            })?;
                    *worn = true;
                }
            }
        }

        let mut applied = Ok(self);

        for (_, (armor_item, worn)) in armor_hashmap.into_iter() {
            applied = applied.and_then(|character| character.with_armor(armor_item, worn));
        }

        applied
    }
}
