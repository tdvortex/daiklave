use crate::{
    character::traits::attributes::{AttributeName, Attributes},
    database::tables::attributes::AttributeNamePostgres,
};
use eyre::Result;
use sqlx::{query, Postgres, Transaction};

#[derive(Debug, Default)]
pub struct AttributesDiff {
    updated_abilities: Vec<(AttributeName, u8)>,
}

impl Attributes {
    pub fn compare_newer(&self, newer: &Self) -> AttributesDiff {
        let mut diff = AttributesDiff::default();

        self.iter().for_each(|attribute| {
            if attribute.dots() != newer.get(attribute.name()).dots() {
                diff.updated_abilities
                    .push((attribute.name(), attribute.dots()));
            }
        });

        diff
    }
}

impl AttributesDiff {
    pub async fn save(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if self.updated_abilities.is_empty() {
            return Ok(());
        }

        let updated_ability_names: Vec<AttributeNamePostgres> = self
            .updated_abilities
            .iter()
            .map(|(name, _)| (*name).into())
            .collect();
        let updated_ability_dots: Vec<i16> = self
            .updated_abilities
            .iter()
            .map(|(_, dots)| *dots as i16)
            .collect();

        query!(
            "
            UPDATE attributes
            SET dots = data.dots
            FROM UNNEST($2::ATTRIBUTENAME[], $3::SMALLINT[]) as data(name, dots)
            WHERE attributes.character_id = $1 AND attributes.name = data.name            
            ",
            character_id,
            &updated_ability_names as &[AttributeNamePostgres],
            &updated_ability_dots as &[i16],
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }
}
