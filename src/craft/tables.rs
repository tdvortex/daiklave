use eyre::{Result, WrapErr};

use crate::character::CharacterBuilder;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "craft_abilities")]
pub(crate) struct CraftAbilityRow {
    character_id: i32,
    focus: String,
    dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "craft_ability_specialties")]
pub(crate) struct CraftAbilitySpecialtyRow {
    character_id: i32,
    focus: String,
    specialty: String,
}

impl CharacterBuilder {
    pub(crate) fn apply_craft(
        mut self,
        craft_ability_rows: Option<Vec<CraftAbilityRow>>,
        specialty_rows: Option<Vec<CraftAbilitySpecialtyRow>>,
    ) -> Result<Self> {
        if craft_ability_rows.is_none() {
            return Ok(self);
        }

        for row in craft_ability_rows.unwrap().into_iter() {
            self = self.with_craft(
                &row.focus.as_str(),
                row.dots
                    .try_into()
                    .wrap_err_with(|| format!("Invalid number of dots: {}", row.dots))?,
            );
        }

        if let Some(rows) = specialty_rows {
            for row in rows {
                self = self.with_craft_specialty(&row.focus.as_str(), row.specialty)?;
            }
        }

        Ok(self)
    }
}
