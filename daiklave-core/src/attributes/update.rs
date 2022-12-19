use super::{tables::{AttributeNamePostgres}, AttributeName};
use crate::attributes::Attributes;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

#[derive(Debug, Default)]
pub struct AttributesDiff {
    updated_attributes: Vec<(AttributeName, u8)>,
}

impl Attributes {
    pub fn compare_newer(&self, newer: &Self) -> AttributesDiff {
        let mut diff = AttributesDiff::default();

        newer.iter().for_each(|attribute| {
            if attribute.dots() != self.get(attribute.name()).dots() {
                diff.updated_attributes.push((attribute.name(), attribute.dots()))
            }
        });

        diff
    }
}


pub async fn update_attributes(
    attributes_diff: AttributesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if attributes_diff.updated_attributes.is_empty() {
        return Ok(());
    }

    let (updated_attribute_names, updated_attribute_dots) = attributes_diff
        .updated_attributes
        .into_iter()
        .fold(
            (Vec::<AttributeNamePostgres>::new(), Vec::<i16>::new()),
            |(mut updated_attribute_names, mut updated_attribute_dots), (name, dots)| {
                updated_attribute_names.push(name.into());
                updated_attribute_dots.push(dots.into());
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
    .await
    .wrap_err("Database error attempting to update attribute dots")?;

    Ok(())
}
