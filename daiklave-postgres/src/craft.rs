use daiklave_core::{character::CharacterBuilder, craft::update::CraftDiff};
use eyre::{Result, WrapErr};
use sqlx::{query, Postgres, Transaction};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "craft_abilities")]
pub struct CraftAbilityRow {
    character_id: i32,
    focus: String,
    dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "craft_ability_specialties")]
pub struct CraftAbilitySpecialtyRow {
    character_id: i32,
    focus: String,
    specialty: String,
}

pub fn apply_craft(
    mut builder: CharacterBuilder,
    craft_ability_rows: Option<Vec<CraftAbilityRow>>,
    specialty_rows: Option<Vec<CraftAbilitySpecialtyRow>>,
) -> Result<CharacterBuilder> {
    if craft_ability_rows.is_none() {
        return Ok(builder);
    }

    for row in craft_ability_rows.unwrap().into_iter() {
        builder = builder.with_craft(
            row.focus.as_str(),
            row.dots
                .try_into()
                .wrap_err_with(|| format!("Invalid number of dots: {}", row.dots))?,
        );
    }

    if let Some(rows) = specialty_rows {
        for row in rows {
            builder = builder.with_craft_specialty(row.focus.as_str(), row.specialty)?;
        }
    }

    Ok(builder)
}

pub async fn update_craft(
    craft_diff: CraftDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    query!(
        "DELETE FROM craft_abilities
        WHERE character_id = $1 AND focus IN (SELECT * FROM UNNEST($2::VARCHAR(255)[]))
        ",
        character_id,
        &craft_diff.removed_foci as &[String]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| "Database error removing craft foci")?;

    let (upserted_foci, upserted_dots) = craft_diff.upserted_foci.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut upserted_foci, mut upserted_dots), (focus, dots)| {
            upserted_foci.push(focus);
            upserted_dots.push(dots as i16);
            (upserted_foci, upserted_dots)
        },
    );

    query!(
        "INSERT INTO craft_abilities(character_id, focus, dots)
        SELECT
            $1 as character_id,
            data.focus as focus,
            data.dots as dots
        FROM UNNEST($2::VARCHAR(255)[], $3::SMALLINT[]) as data(focus, dots)
        ON CONFLICT (character_id, focus) DO UPDATE SET dots = EXCLUDED.dots
        ",
        character_id,
        &upserted_foci as &[String],
        &upserted_dots as &[i16]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| "Database error upserting craft foci")?;

    let (foci_with_removed_specialty, removed_specialty) =
        craft_diff.removed_specialties.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut foci_with_removed_specialty, mut removed_specialty), (focus, specialty)| {
                foci_with_removed_specialty.push(focus);
                removed_specialty.push(specialty);
                (foci_with_removed_specialty, removed_specialty)
            },
        );

    query!(
        "DELETE FROM craft_ability_specialties
        WHERE character_id = $1 AND (focus, specialty) IN 
        (
            SELECT
                data.focus as focus,
                data.specialty as specialty
            FROM UNNEST($2::VARCHAR(255)[], $3::VARCHAR(255)[]) as data(focus, specialty)
        )",
        character_id,
        &foci_with_removed_specialty as &[String],
        &removed_specialty as &[String]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| "Database error deleting craft specialties")?;

    let (foci_with_added_specialty, added_specialty) =
        craft_diff.added_specialties.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut foci_with_added_specialty, mut added_specialty), (focus, specialty)| {
                foci_with_added_specialty.push(focus);
                added_specialty.push(specialty);
                (foci_with_added_specialty, added_specialty)
            },
        );

    query!(
        "INSERT INTO craft_ability_specialties(character_id, focus, specialty)
        SELECT
            $1 as character_id,
            data.focus as focus,
            data.specialty as specialty
        FROM UNNEST($2::VARCHAR(255)[], $3::VARCHAR(255)[]) as data(focus, specialty)
        ",
        character_id,
        &foci_with_added_specialty as &[String],
        &added_specialty as &[String]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| "Database error deleting craft specialties")?;

    Ok(())
}
