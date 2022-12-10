use crate::armor::ArmorItem;
use crate::{armor::tables::ArmorTagTypePostgres, custom::DataSource};
use eyre::{Result, WrapErr};
use sqlx::{query, Postgres, Transaction};

pub(crate) async fn create_armor_item_transaction(
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
        if armor_item.data_source() == &DataSource::Custom(None) {
            output.push(
                create_armor_item_transaction(transaction, armor_item, Some(character_id))
                    .await
                    .wrap_err("Database error creating new custom armor item")?,
            );
        } else {
            let maybe_creator_id = armor_item.data_source().creator_id();
            output.push(
                create_armor_item_transaction(transaction, armor_item, maybe_creator_id)
                    .await
                    .wrap_err("Database error creating armor item")?,
            );
        }
    }

    Ok(output)
}
