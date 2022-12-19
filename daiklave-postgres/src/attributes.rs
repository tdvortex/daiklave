use daiklave_core::{
    attributes::{AttributeName, AttributesDiff},
    character::CharacterBuilder,
};
use eyre::{Result, WrapErr};
use sqlx::{postgres::PgHasArrayType, query, Postgres, Transaction};

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ATTRIBUTENAME", rename_all = "UPPERCASE")]
pub enum AttributeNamePostgres {
    Strength,
    Dexterity,
    Stamina,
    Charisma,
    Manipulation,
    Appearance,
    Perception,
    Intelligence,
    Wits,
}

impl From<AttributeNamePostgres> for AttributeName {
    fn from(value: AttributeNamePostgres) -> Self {
        match value {
            AttributeNamePostgres::Strength => Self::Strength,
            AttributeNamePostgres::Dexterity => Self::Dexterity,
            AttributeNamePostgres::Stamina => Self::Stamina,
            AttributeNamePostgres::Charisma => Self::Charisma,
            AttributeNamePostgres::Manipulation => Self::Manipulation,
            AttributeNamePostgres::Appearance => Self::Appearance,
            AttributeNamePostgres::Perception => Self::Perception,
            AttributeNamePostgres::Intelligence => Self::Intelligence,
            AttributeNamePostgres::Wits => Self::Wits,
        }
    }
}

impl From<AttributeName> for AttributeNamePostgres {
    fn from(value: AttributeName) -> Self {
        match value {
            AttributeName::Strength => Self::Strength,
            AttributeName::Dexterity => Self::Dexterity,
            AttributeName::Stamina => Self::Stamina,
            AttributeName::Charisma => Self::Charisma,
            AttributeName::Manipulation => Self::Manipulation,
            AttributeName::Appearance => Self::Appearance,
            AttributeName::Perception => Self::Perception,
            AttributeName::Intelligence => Self::Intelligence,
            AttributeName::Wits => Self::Wits,
        }
    }
}

impl PgHasArrayType for AttributeNamePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ATTRIBUTENAME")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "attributes")]
pub struct AttributeRow {
    pub character_id: i32,
    pub name: AttributeNamePostgres,
    pub dots: i16,
}

fn apply_attribute_row(
    builder: CharacterBuilder,
    attribute_row: AttributeRow,
) -> Result<CharacterBuilder> {
    let attribute_name = attribute_row.name.into();
    let value = attribute_row
        .dots
        .try_into()
        .wrap_err_with(|| format!("Invalid number of dots: {}", attribute_row.dots))?;

    builder.with_attribute(attribute_name, value)
}

pub fn apply_attribute_rows(
    builder: CharacterBuilder,
    attribute_rows: Vec<AttributeRow>,
) -> Result<CharacterBuilder> {
    attribute_rows
        .into_iter()
        .fold(Ok(builder), |output, attribute_row| {
            output.and_then(|character| apply_attribute_row(character, attribute_row))
        })
}

pub async fn update_attributes(
    attributes_diff: AttributesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if attributes_diff.updated_attributes.is_empty() {
        return Ok(());
    }

    let (updated_attribute_names, updated_attribute_dots) =
        attributes_diff.updated_attributes.into_iter().fold(
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
