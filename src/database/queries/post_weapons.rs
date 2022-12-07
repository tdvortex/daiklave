use ::eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::{character::traits::weapons::Weapon, database::tables::weapons::WeaponTagPostgres};

pub async fn post_weapons_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    mut weapons: Vec<Weapon>,
) -> Result<Vec<i32>> {
    let (names, tags, creator_ids): (Vec<String>, Vec<Vec<WeaponTagPostgres>>, Vec<Option<i32>>) =
        weapons
            .into_iter()
            .filter(|weapon| weapon.id().is_none())
            .map(|weapon| {
                (
                    weapon.name().to_owned(),
                    weapon
                        .tags()
                        .into_iter()
                        .map(|tag| tag.into())
                        .collect::<Vec<WeaponTagPostgres>>(),
                    weapon.creator_id(),
                )
            })
            .fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut names, mut tags, mut creator_ids), (name, tag_list, creator_id)| {
                    names.push(name);
                    tags.push(tag_list);
                    creator_ids.push(creator_id);
                    (names, tags, creator_ids)
                },
            );

    query!(
          "INSERT INTO weapons(name, tags, creator_id)
          SELECT 
               data.name as name,
               data.input_tags as tags,
               data.creator_id as creator_id
          FROM UNNEST($1::VARCHAR(255)[], $2::WEAPONTAG[], $3::INTEGER[]) as data(name, input_tags, creator_id)
          RETURNING id
          ",
          &names as &[&str],
          tags as Vec<WeaponTagPostgres>,
          &creator_ids as &[Option<i32>],
     ).fetch_all(&mut *transaction).await?.iter().map(|record| record.id).collect()
}