use eyre::Context;
use ::eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::custom::DataSource;
use crate::weapons::tables::WeaponTagPostgres;
use crate::weapons::Weapon;

pub(crate) async fn create_weapons_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    weapons: Vec<Weapon>,
    character_id: i32,
) -> Result<Vec<i32>> {
    let mut output = Vec::new();
    for weapon in weapons.into_iter() {
        output.push(
            create_weapon_transaction(transaction, weapon, character_id)
                .await
                .wrap_err("Database error creating new custom weapon")?,
        );
    }

    Ok(output)
}

pub(crate) async fn create_weapon_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    weapon: Weapon,
    character_id: i32,
) -> Result<i32> {
    let (title, number, creator_id) = match weapon.data_source() {
        DataSource::Custom(None) => (None, None, Some(character_id)),
        _ => (
            weapon.data_source.book_title(),
            weapon.data_source().page_number(),
            weapon.data_source().creator_id(),
        ),
    };

    Ok(query!(
        "INSERT INTO weapons(name, tags, book_title, page_number, creator_id)
        VALUES (
            $1::VARCHAR(255),
            $2::WEAPONTAG[],
            $3::VARCHAR(255),
            $4::SMALLINT,
            $5::INTEGER
        )
        RETURNING id",
        weapon.name(),
        &weapon
            .tags()
            .into_iter()
            .map(|tag| tag.into())
            .collect::<Vec<WeaponTagPostgres>>() as &[WeaponTagPostgres],
        title as Option<&str>,
        number as Option<i16>,
        creator_id as Option<i32>,
    )
    .fetch_one(&mut *transaction)
    .await.wrap_err_with(|| format!("Database error when inserting weapon name {}", weapon.name()))?
    .id)
}
