use daiklave_core::character::{CharacterBaseDiff, CharacterBuilder, ExperiencePoints, Willpower};
use eyre::{Result, WrapErr};
use sqlx::{query, Postgres, Transaction};

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EXALTTYPE", rename_all = "UPPERCASE")]
pub enum ExaltTypePostgres {
    Solar,
    Lunar,
    DragonBlooded,
}

#[derive(Debug)]
pub struct CharacterRow {
    pub id: i32,
    pub player_id: i32,
    pub campaign_id: Option<i32>,
    pub name: String,
    pub concept: Option<String>,
    pub exalt_type: Option<ExaltTypePostgres>,
    pub current_willpower: i16,
    pub max_willpower: i16,
    pub current_experience: i16,
    pub total_experience: i16,
}

impl sqlx::Type<sqlx::Postgres> for CharacterRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("characters")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CharacterRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let player_id = decoder.try_decode::<i32>()?;
        let campaign_id = decoder.try_decode::<Option<i32>>()?;
        let name = decoder.try_decode::<String>()?;
        let concept = decoder.try_decode::<Option<String>>()?;
        let exalt_type = decoder.try_decode::<Option<ExaltTypePostgres>>()?;
        let current_willpower = decoder.try_decode::<i16>()?;
        let max_willpower = decoder.try_decode::<i16>()?;
        let current_experience = decoder.try_decode::<i16>()?;
        let total_experience = decoder.try_decode::<i16>()?;

        Ok(Self {
            id,
            player_id,
            campaign_id,
            name,
            concept,
            exalt_type,
            current_willpower,
            max_willpower,
            current_experience,
            total_experience,
        })
    }
}

pub fn apply_character_row(
    builder: CharacterBuilder,
    character_row: CharacterRow,
) -> Result<CharacterBuilder> {
    let willpower = Willpower {
        current: character_row.current_willpower.try_into()?,
        maximum: character_row.max_willpower.try_into()?,
    };

    let experience = ExperiencePoints {
        current: character_row.current_experience.try_into()?,
        total: character_row.total_experience.try_into()?,
    };

    let mut applied = builder
        .with_database_id(character_row.id)
        .with_name(character_row.name)
        .with_willpower(willpower)
        .with_experience(experience);

    if let Some(text) = character_row.concept {
        applied = applied.with_concept(text);
    }

    Ok(applied)
}

pub async fn update_base_character(
    base_character_diff: CharacterBaseDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if base_character_diff.0.is_none() {
        return Ok(());
    }

    let (
        name,
        maybe_concept,
        current_willpower,
        maximum_willpower,
        current_experience,
        total_experience,
        _initiative,
    ) = base_character_diff.0.as_ref().unwrap();

    query!("
        UPDATE characters
        SET name = $2, concept = $3, current_willpower = $4, max_willpower = $5, current_experience = $6, total_experience = $7
        WHERE id = $1",
        character_id, name.as_ref() as &str, maybe_concept.as_deref(), current_willpower, maximum_willpower, current_experience, total_experience
    ).execute(&mut *transaction).await.wrap_err_with(|| format!("Failed to update character: {:?}", base_character_diff.0.as_ref().unwrap()))?;

    Ok(())
}