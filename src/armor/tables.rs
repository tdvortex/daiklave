use eyre::{eyre, Context, Result};
use std::collections::HashMap;

use sqlx::postgres::PgHasArrayType;

use crate::armor::{ArmorItem, ArmorTag};
use crate::character::CharacterBuilder;

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

impl CharacterBuilder {
    pub(crate) fn apply_armor_rows(
        mut self,
        armor_owned: Option<Vec<ArmorRow>>,
        armor_tags: Option<Vec<ArmorTagRow>>,
        armor_worn: Option<Vec<ArmorWornRow>>,
    ) -> Result<Self> {
        if armor_owned.is_none() || armor_worn.is_none() {
            return Ok(self);
        }

        let armor_worn = armor_worn.unwrap();

        let mut armor_rows_hashmap = HashMap::new();

        for armor_item in armor_owned.unwrap().into_iter() {
            armor_rows_hashmap.insert(armor_item.id, (armor_item, Vec::new()));
        }

        if let Some(tags) = armor_tags {
            for tag_row in tags.into_iter() {
                armor_rows_hashmap
                    .get_mut(&tag_row.armor_id)
                    .ok_or_else(|| eyre!("No armor item with id {} for tag", tag_row.armor_id))?
                    .1
                    .push(tag_row);
            }
        }

        let mut armor_items_hashmap = HashMap::new();
        for (armor_id, (armor_row, armor_tags)) in armor_rows_hashmap.into_iter() {
            let mut builder = if armor_row.book_title.is_some()
                && armor_row.page_number.is_some()
                && armor_row.creator_id.is_none()
            {
                ArmorItem::create_from_book(
                    armor_row.book_title.unwrap(),
                    armor_row.page_number.unwrap(),
                )
            } else if armor_row.book_title.is_none()
                && armor_row.book_title.is_none()
                && armor_row.creator_id.is_some()
            {
                ArmorItem::create_custom(armor_row.creator_id)
            } else {
                return Err(eyre!(
                    "Database error: inconsistent data source for armor item {}",
                    armor_id
                ));
            };

            builder = builder.with_id(armor_id).with_name(armor_row.name);

            for tag_row in armor_tags.into_iter() {
                builder = builder.with_tag(tag_row.tag_type.into());
            }

            let armor_item = builder
                .build()
                .wrap_err_with(|| format!("Failed to build armor item {} from tags", armor_id))?;

            armor_items_hashmap.insert(armor_id, armor_item);
        }

        for worn_row in armor_worn {
            let armor_item = armor_items_hashmap
                .remove(&worn_row.armor_id)
                .ok_or_else(|| {
                    eyre!(
                        "Missing item row {} for character {}",
                        worn_row.armor_id,
                        worn_row.character_id
                    )
                })?;
            let worn = worn_row.worn;
            self = self.with_armor(armor_item, worn).wrap_err_with(|| {
                format!(
                    "Could not give armor item {} to character {}",
                    worn_row.armor_id, worn_row.character_id
                )
            })?;
        }

        Ok(self)
    }
}
