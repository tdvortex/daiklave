use crate::character::{builder::CharacterBuilder, traits::attributes::AttributeName};
use eyre::Result;

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
    fn from(val: AttributeNamePostgres) -> Self {
        match val {
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "attributes")]
pub struct AttributeRow {
    pub character_id: i32,
    pub name: AttributeNamePostgres,
    pub dots: i16,
}

impl CharacterBuilder {
    fn apply_attribute_row(&mut self, attribute_row: AttributeRow) -> Result<&mut Self> {
        let attribute_name = attribute_row.name.into();
        let value = attribute_row.dots.try_into()?;

        self.with_attribute(attribute_name, value)
    }

    pub fn apply_attribute_rows(&mut self, attribute_rows: Vec<AttributeRow>) -> Result<&mut Self> {
        attribute_rows
            .into_iter()
            .fold(Ok(self), |output, attribute_row| {
                output.and_then(|character| character.apply_attribute_row(attribute_row))
            })
    }
}
