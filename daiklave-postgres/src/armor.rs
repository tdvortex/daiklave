use std::collections::HashMap;

use daiklave_core::{
    armor::{ArmorDiff, ArmorItem, ArmorTag},
    character::CharacterBuilder,
    data_source::DataSource,
};
use eyre::{eyre, Result, WrapErr};
use sqlx::{postgres::PgHasArrayType, query, Postgres, Transaction};

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

pub fn apply_armor_rows(
    mut builder: CharacterBuilder,
    armor_owned: Option<Vec<ArmorRow>>,
    armor_tags: Option<Vec<ArmorTagRow>>,
    armor_worn: Option<Vec<ArmorWornRow>>,
) -> Result<CharacterBuilder> {
    if armor_owned.is_none() || armor_worn.is_none() {
        return Ok(builder);
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
        let mut armor_builder = if armor_row.book_title.is_some()
            && armor_row.page_number.is_some()
            && armor_row.creator_id.is_none()
        {
            ArmorItem::from_book(
                armor_id,
                armor_row.book_title.unwrap(),
                armor_row.page_number.unwrap(),
            )
        } else if armor_row.book_title.is_none()
            && armor_row.book_title.is_none()
            && armor_row.creator_id.is_some()
        {
            ArmorItem::custom(armor_id, builder.id())
        } else {
            return Err(eyre!(
                "Database error: inconsistent data source for armor item {}",
                armor_id
            ));
        };

        armor_builder = armor_builder
            .with_database_id(armor_id)
            .with_name(armor_row.name);

        for tag_row in armor_tags.into_iter() {
            armor_builder = armor_builder.with_tag(tag_row.tag_type.into());
        }

        let armor_item = armor_builder
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
        builder = builder.with_armor(armor_item, worn);
    }

    Ok(builder)
}

pub async fn create_armor_item_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    armor_item: ArmorItem,
    creator_id: Option<i32>,
) -> Result<i32> {
    let item = query!(
        "INSERT INTO armor(name, book_title, page_number, creator_id)
        VALUES (
            $1::VARCHAR(255),
            $2::VARCHAR(255),
            $3::SMALLINT,
            $4::INTEGER
        )
        RETURNING id",
        armor_item.name() as &str,
        armor_item.data_source().book_title() as Option<&str>,
        armor_item.data_source().page_number() as Option<i16>,
        creator_id
    )
    .fetch_one(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error creating armor item with name '{}'",
            armor_item.name()
        )
    })?
    .id;

    let tags = armor_item
        .tags()
        .into_iter()
        .map(|tag| tag.into())
        .collect::<Vec<ArmorTagTypePostgres>>();

    query!(
        "INSERT INTO armor_tags(armor_id, tag_type)
        SELECT
            $1::INTEGER as armor_id,
            data.tag_type
        FROM UNNEST($2::ARMORTAGTYPE[]) as data(tag_type)",
        item as i32,
        &tags as &[ArmorTagTypePostgres]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| format!("Database error creating armor tags for armor item {}", item))?;

    Ok(item)
}

pub(crate) async fn create_armor_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    armor: Vec<ArmorItem>,
    character_id: i32,
) -> Result<Vec<i32>> {
    let mut output = Vec::new();
    for armor_item in armor.into_iter() {
        if let DataSource::Custom(_) = armor_item.data_source() {
            output.push(
                create_armor_item_transaction(transaction, armor_item, Some(character_id))
                    .await
                    .wrap_err("Database error creating custom armor item")?,
            );
        } else {
            output.push(
                create_armor_item_transaction(transaction, armor_item, None)
                    .await
                    .wrap_err("Database error creating book referenced armor item")?,
            );
        }
    }

    Ok(output)
}

pub async fn update_armor(
    armor_diff: ArmorDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if armor_diff.noop {
        return Ok(());
    }

    query!(
        "DELETE FROM character_armor
        WHERE character_id = $1
        ",
        character_id
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error deleting armor owned/worn")?;

    let (new_items, mut new_items_equipped) = armor_diff.insert_items.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut new_items, mut new_items_equipped), (item, equipped)| {
            new_items.push(item);
            new_items_equipped.push(equipped);
            (new_items, new_items_equipped)
        },
    );

    let mut new_ids = create_armor_transaction(transaction, new_items, character_id)
        .await
        .wrap_err("Error trying to create new armor items")?;

    let (mut ids, mut ids_equipped) = armor_diff.owned_items.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut ids, mut ids_equipped), id| {
            ids.push(id);
            ids_equipped.push(Some(id) == armor_diff.worn_item);
            (ids, ids_equipped)
        },
    );

    ids.append(&mut new_ids);
    ids_equipped.append(&mut new_items_equipped);

    query!(
        "INSERT INTO character_armor(character_id, armor_id, worn)
        SELECT
            $1::INTEGER as character_id,
            data.armor_id as armor_id,
            data.worn as worn
        FROM UNNEST($2::INTEGER[], $3::BOOLEAN[]) as data(armor_id, worn)",
        character_id,
        &ids as &[i32],
        &ids_equipped as &[bool]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error trying to armor owned/worn rows")?;

    Ok(())
}
