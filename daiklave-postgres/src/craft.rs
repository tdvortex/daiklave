use daiklave_core::character::CharacterBuilder;
use eyre::{WrapErr, Result};

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
