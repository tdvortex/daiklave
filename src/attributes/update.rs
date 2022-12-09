use super::tables::{AttributeNamePostgres, AttributeUpdate};
use crate::attributes::Attributes;
use eyre::Result;
use sqlx::{query, Postgres, Transaction};

#[derive(Debug, Default)]
pub struct AttributesDiff {
    updated_attributes: Vec<AttributeUpdate>,
}

impl Attributes {
    pub fn compare_newer(&self, newer: &Self) -> AttributesDiff {
        let mut diff = AttributesDiff::default();

        self.iter().for_each(|attribute| {
            if attribute.dots() != newer.get(attribute.name()).dots() {
                diff.updated_attributes.push(attribute.into());
            }
        });

        diff
    }
}

impl AttributesDiff {
    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if self.updated_attributes.is_empty() {
            return Ok(());
        }

        let (updated_attribute_names, updated_attribute_dots) = self
            .updated_attributes
            .into_iter()
            .map(|update| update.into_tuple())
            .fold(
                (Vec::new(), Vec::new()),
                |(mut updated_attribute_names, mut updated_attribute_dots), (name, dots)| {
                    updated_attribute_names.push(name);
                    updated_attribute_dots.push(dots);
                    (updated_attribute_names, updated_attribute_dots)
                },
            );

        query!(
            "
            UPDATE attributes
            SET dots = data.dots
            FROM UNNEST($2::ATTRIBUTENAME[], $3::SMALLINT[]) as data(name, dots)
            WHERE attributes.character_id = $1 AND attributes.name = data.name            
            ",
            character_id,
            &updated_attribute_names as &[AttributeNamePostgres],
            &updated_attribute_dots as &[i16],
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }
}
