use crate::{character::traits::armor::ArmorItem, database::tables::armor::ArmorTagPostgres};
use eyre::Result;
use sqlx::{query, PgPool, Postgres, Transaction};

pub async fn post_armor_item_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    armor_item: ArmorItem,
) -> Result<i32> {
    Ok(query!(
        "INSERT INTO armor(name, tags, creator_id)
        VALUES (
            $1::VARCHAR(255),
            $2::ARMORTAG[],
            $3::INTEGER
        )
        RETURNING id",
        armor_item.name(),
        &armor_item
            .tags()
            .into_iter()
            .map(|tag| tag.into())
            .collect::<Vec<ArmorTagPostgres>>() as &[ArmorTagPostgres],
        armor_item.creator_id(),
    )
    .fetch_one(&mut *transaction)
    .await?
    .id)
}

pub async fn post_armor(pool: &PgPool, armor: Vec<ArmorItem>) -> Result<Vec<i32>> {
    let mut transaction = pool.begin().await?;

    let ids = post_armor_transaction(&mut transaction, armor).await?;

    transaction.commit().await?;

    Ok(ids)
}

pub(crate) async fn post_armor_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    armor: Vec<ArmorItem>,
) -> Result<Vec<i32>> {
    let mut output = Vec::new();
    for armor_item in armor.into_iter() {
        output.push(post_armor_item_transaction(transaction, armor_item).await?);
    }

    Ok(output)
}
