use ::eyre::Result;
use eyre::Context;
use sqlx::{query, Postgres, Transaction};

use crate::data_source::DataSource;
use crate::weapons::Weapon;

use super::tables::{RangeBandPostgres, WeaponTagTypePostgres};
use super::WeaponTag;

pub(crate) async fn create_weapons_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    weapons: Vec<Weapon>,
    character_id: i32,
) -> Result<Vec<i32>> {
    let mut output = Vec::new();
    for weapon in weapons.into_iter() {
        if let DataSource::Custom(_) = weapon.data_source() {
            output.push(
                create_weapon_transaction(transaction, weapon, Some(character_id))
                    .await
                    .wrap_err("Database error creating new custom weapon")?,
            );
        } else {
            output.push(
                create_weapon_transaction(transaction, weapon, None)
                    .await
                    .wrap_err("Database error creating new book referenced weapon")?,
            );
        }
    }

    Ok(output)
}

pub(crate) async fn create_weapon_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    weapon: Weapon,
    creator_id: Option<i32>,
) -> Result<i32> {
    let weapon_id = query!(
        "INSERT INTO weapons(name, book_title, page_number, creator_id)
        VALUES (
            $1::VARCHAR(255),
            $2::VARCHAR(255),
            $3::SMALLINT,
            $4::INTEGER
        )
        RETURNING id",
        weapon.name() as &str,
        weapon.data_source().book_title() as Option<&str>,
        weapon.data_source().page_number() as Option<i16>,
        creator_id
    )
    .fetch_one(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error creating weapon with name '{}'",
            weapon.name()
        )
    })?
    .id;

    let (tag_types, ranges, styles) = weapon.tags().into_iter().fold(
        (
            Vec::<WeaponTagTypePostgres>::new(),
            Vec::<Option<RangeBandPostgres>>::new(),
            Vec::<Option<String>>::new(),
        ),
        |(mut tag_types, mut ranges, mut styles), tag| {
            match tag {
                WeaponTag::Archery(range) => {
                    ranges.push(Some(range.into()));
                    styles.push(None);
                    tag_types.push(WeaponTagTypePostgres::Archery);
                }
                WeaponTag::Thrown(range) => {
                    ranges.push(Some(range.into()));
                    styles.push(None);
                    tag_types.push(WeaponTagTypePostgres::Thrown);
                }
                WeaponTag::MartialArts(style) => {
                    ranges.push(None);
                    styles.push(Some(style));
                    tag_types.push(WeaponTagTypePostgres::MartialArts);
                }
                other => {
                    ranges.push(None);
                    styles.push(None);
                    tag_types.push(other.into())
                }
            }
            (tag_types, ranges, styles)
        },
    );

    query!(
        "INSERT INTO weapon_tags(weapon_id, tag_type, max_range, martial_arts_style)
        SELECT
            $1::INTEGER as weapon_id,
            data.tag_type,
            data.max_range,
            data.martial_arts_style
        FROM UNNEST($2::WEAPONTAGTYPE[], $3::RANGEBAND[], $4::VARCHAR(255)[]) as data(tag_type, max_range, martial_arts_style)",
        weapon_id as i32,
        &tag_types as &[WeaponTagTypePostgres],
        &ranges as &[Option<RangeBandPostgres>],
        &styles as &[Option<String>]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| format!("Database error creating weapon tags for weapon {}", weapon_id))?;

    Ok(weapon_id)
}
