use ::eyre::Result;
use sqlx::{query, PgPool, Postgres, Transaction};

use crate::weapons::tables::WeaponTagPostgres;
use crate::weapons::Weapon;

pub async fn create_weapons(pool: &PgPool, weapons: Vec<Weapon>) -> Result<Vec<i32>> {
    let mut transaction = pool.begin().await?;

    let ids = create_weapons_transaction(&mut transaction, weapons).await?;

    transaction.commit().await?;

    Ok(ids)
}

pub(crate) async fn create_weapons_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    weapons: Vec<Weapon>,
) -> Result<Vec<i32>> {
    let mut output = Vec::new();
    for weapon in weapons.into_iter() {
        output.push(create_weapon_transaction(transaction, weapon).await?);
    }

    Ok(output)
}

pub(crate) async fn create_weapon_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    weapon: Weapon,
) -> Result<i32> {
    Ok(query!(
        "INSERT INTO weapons(name, tags, creator_id)
        VALUES (
            $1::VARCHAR(255),
            $2::WEAPONTAG[],
            $3::INTEGER
        )
        RETURNING id",
        weapon.name(),
        &weapon
            .tags()
            .into_iter()
            .map(|tag| tag.into())
            .collect::<Vec<WeaponTagPostgres>>() as &[WeaponTagPostgres],
        weapon.creator_id(),
    )
    .fetch_one(&mut *transaction)
    .await?
    .id)
}