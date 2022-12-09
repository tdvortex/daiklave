use crate::character::{traits::attributes::AttributeName, CharacterBuilder};
use eyre::Result;
use sqlx::postgres::PgHasArrayType;

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

impl CharacterBuilder {
    fn apply_attribute_row(self, attribute_row: AttributeRow) -> Result<Self> {
        let attribute_name = attribute_row.name.into();
        let value = attribute_row.dots.try_into()?;

        self.with_attribute(attribute_name, value)
    }

    pub fn apply_attribute_rows(self, attribute_rows: Vec<AttributeRow>) -> Result<Self> {
        attribute_rows
            .into_iter()
            .fold(Ok(self), |output, attribute_row| {
                output.and_then(|character| character.apply_attribute_row(attribute_row))
            })
    }
}
