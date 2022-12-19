use daiklave_core::{
    abilities::{AbilitiesDiff, AbilityNameNoSubskill, AbilityNameVanilla},
    character::CharacterBuilder,
};
use eyre::{Result, WrapErr};
use sqlx::{postgres::PgHasArrayType, query, Postgres, Transaction};

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "ABILITYNAMEVANILLA", rename_all = "UPPERCASE")]
pub enum AbilityNameVanillaPostgres {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Dodge,
    Integrity,
    Investigation,
    Larceny,
    Linguistics,
    Lore,
    Medicine,
    Melee,
    Occult,
    Performance,
    Presence,
    Resistance,
    Ride,
    Sail,
    Socialize,
    Stealth,
    Survival,
    Thrown,
    War,
}

impl PgHasArrayType for AbilityNameVanillaPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ABILITYNAMEVANILLA")
    }
}

impl From<AbilityNameVanillaPostgres> for AbilityNameNoSubskill {
    fn from(ability_name_postgres: AbilityNameVanillaPostgres) -> Self {
        match ability_name_postgres {
            AbilityNameVanillaPostgres::Archery => Self::Archery,
            AbilityNameVanillaPostgres::Athletics => Self::Athletics,
            AbilityNameVanillaPostgres::Awareness => Self::Awareness,
            AbilityNameVanillaPostgres::Brawl => Self::Brawl,
            AbilityNameVanillaPostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanillaPostgres::Dodge => Self::Dodge,
            AbilityNameVanillaPostgres::Integrity => Self::Integrity,
            AbilityNameVanillaPostgres::Investigation => Self::Investigation,
            AbilityNameVanillaPostgres::Larceny => Self::Larceny,
            AbilityNameVanillaPostgres::Linguistics => Self::Linguistics,
            AbilityNameVanillaPostgres::Lore => Self::Lore,
            AbilityNameVanillaPostgres::Medicine => Self::Medicine,
            AbilityNameVanillaPostgres::Melee => Self::Melee,
            AbilityNameVanillaPostgres::Occult => Self::Occult,
            AbilityNameVanillaPostgres::Performance => Self::Performance,
            AbilityNameVanillaPostgres::Presence => Self::Presence,
            AbilityNameVanillaPostgres::Resistance => Self::Resistance,
            AbilityNameVanillaPostgres::Ride => Self::Ride,
            AbilityNameVanillaPostgres::Sail => Self::Sail,
            AbilityNameVanillaPostgres::Socialize => Self::Socialize,
            AbilityNameVanillaPostgres::Stealth => Self::Stealth,
            AbilityNameVanillaPostgres::Survival => Self::Survival,
            AbilityNameVanillaPostgres::Thrown => Self::Thrown,
            AbilityNameVanillaPostgres::War => Self::War,
        }
    }
}

impl From<AbilityNameVanillaPostgres> for AbilityNameVanilla {
    fn from(ability_name_postgres: AbilityNameVanillaPostgres) -> Self {
        match ability_name_postgres {
            AbilityNameVanillaPostgres::Archery => Self::Archery,
            AbilityNameVanillaPostgres::Athletics => Self::Athletics,
            AbilityNameVanillaPostgres::Awareness => Self::Awareness,
            AbilityNameVanillaPostgres::Brawl => Self::Brawl,
            AbilityNameVanillaPostgres::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanillaPostgres::Dodge => Self::Dodge,
            AbilityNameVanillaPostgres::Integrity => Self::Integrity,
            AbilityNameVanillaPostgres::Investigation => Self::Investigation,
            AbilityNameVanillaPostgres::Larceny => Self::Larceny,
            AbilityNameVanillaPostgres::Linguistics => Self::Linguistics,
            AbilityNameVanillaPostgres::Lore => Self::Lore,
            AbilityNameVanillaPostgres::Medicine => Self::Medicine,
            AbilityNameVanillaPostgres::Melee => Self::Melee,
            AbilityNameVanillaPostgres::Occult => Self::Occult,
            AbilityNameVanillaPostgres::Performance => Self::Performance,
            AbilityNameVanillaPostgres::Presence => Self::Presence,
            AbilityNameVanillaPostgres::Resistance => Self::Resistance,
            AbilityNameVanillaPostgres::Ride => Self::Ride,
            AbilityNameVanillaPostgres::Sail => Self::Sail,
            AbilityNameVanillaPostgres::Socialize => Self::Socialize,
            AbilityNameVanillaPostgres::Stealth => Self::Stealth,
            AbilityNameVanillaPostgres::Survival => Self::Survival,
            AbilityNameVanillaPostgres::Thrown => Self::Thrown,
            AbilityNameVanillaPostgres::War => Self::War,
        }
    }
}

impl From<AbilityNameVanilla> for AbilityNameVanillaPostgres {
    fn from(ability_name: AbilityNameVanilla) -> Self {
        match ability_name {
            AbilityNameVanilla::Archery => Self::Archery,
            AbilityNameVanilla::Athletics => Self::Athletics,
            AbilityNameVanilla::Awareness => Self::Awareness,
            AbilityNameVanilla::Brawl => Self::Brawl,
            AbilityNameVanilla::Bureaucracy => Self::Bureaucracy,
            AbilityNameVanilla::Dodge => Self::Dodge,
            AbilityNameVanilla::Integrity => Self::Integrity,
            AbilityNameVanilla::Investigation => Self::Investigation,
            AbilityNameVanilla::Larceny => Self::Larceny,
            AbilityNameVanilla::Linguistics => Self::Linguistics,
            AbilityNameVanilla::Lore => Self::Lore,
            AbilityNameVanilla::Medicine => Self::Medicine,
            AbilityNameVanilla::Melee => Self::Melee,
            AbilityNameVanilla::Occult => Self::Occult,
            AbilityNameVanilla::Performance => Self::Performance,
            AbilityNameVanilla::Presence => Self::Presence,
            AbilityNameVanilla::Resistance => Self::Resistance,
            AbilityNameVanilla::Ride => Self::Ride,
            AbilityNameVanilla::Sail => Self::Sail,
            AbilityNameVanilla::Socialize => Self::Socialize,
            AbilityNameVanilla::Stealth => Self::Stealth,
            AbilityNameVanilla::Survival => Self::Survival,
            AbilityNameVanilla::Thrown => Self::Thrown,
            AbilityNameVanilla::War => Self::War,
        }
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "abilities")]
pub struct AbilityRow {
    pub character_id: i32,
    pub name: AbilityNameVanillaPostgres,
    pub dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "specialties")]
pub struct SpecialtyRow {
    pub character_id: i32,
    pub name: AbilityNameVanillaPostgres,
    pub specialty: String,
}

impl PgHasArrayType for SpecialtyRow {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_specialties")
    }
}

pub fn apply_abilities_and_specialties_rows(
    mut builder: CharacterBuilder,
    abilities_rows: Vec<AbilityRow>,
    specialty_rows: Option<Vec<SpecialtyRow>>,
) -> Result<CharacterBuilder> {
    for row in abilities_rows.into_iter() {
        builder = builder.with_ability(
            row.name.into(),
            row.dots
                .try_into()
                .wrap_err_with(|| format!("Invalid number of dots: {}", row.dots))?,
        );
    }

    if let Some(rows) = specialty_rows {
        for row in rows.into_iter() {
            builder = builder.with_specialty(row.name.into(), row.specialty)?;
        }
    }

    Ok(builder)
}

async fn update_vanilla_abilities(
    abilities_diff: &AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    let (mut names_to_update, mut dots_to_update) =
        (Vec::<AbilityNameVanillaPostgres>::new(), Vec::new());

    for (name_vanilla, dots) in abilities_diff.abilities_to_modify.iter() {
        names_to_update.push((*name_vanilla).into());
        dots_to_update.push((*dots).into());
    }

    if !names_to_update.is_empty() {
        query!(
            "UPDATE abilities
            SET dots = data.dots
            FROM UNNEST($2::ABILITYNAMEVANILLA[], $3::SMALLINT[]) as data(name, dots)
            WHERE abilities.character_id = $1 AND abilities.name = data.name",
            character_id,
            &names_to_update as &[AbilityNameVanillaPostgres],
            &dots_to_update as &[i16]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err("Database error attempting to update non-Craft, non-MartialArts abilities")?;
    }

    Ok(())
}

async fn remove_specialties(
    abilities_diff: &AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    let ability_name_with_specialty_to_remove: Vec<AbilityNameVanillaPostgres> = abilities_diff
        .specialties_to_remove
        .iter()
        .map(|(ability_name, _)| (*ability_name).into())
        .collect();

    let specialty_name_to_remove: Vec<&str> = abilities_diff
        .specialties_to_remove
        .iter()
        .map(|(_, specialty)| specialty.as_str())
        .collect();

    query!(
        "
        DELETE FROM specialties
        WHERE (specialties.character_id, specialties.ability_name, specialties.specialty) IN
        (
            SELECT
                $1::INTEGER as character_id,
                data.ability_name as ability_name,
                data.specialty as specialty
            FROM UNNEST($2::ABILITYNAMEVANILLA[], $3::VARCHAR(255)[]) AS data(ability_name, specialty)
        )",
        character_id as i32,
        &ability_name_with_specialty_to_remove as &[AbilityNameVanillaPostgres],
        &specialty_name_to_remove as &[&str]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error attempting to remove specialties")?;

    Ok(())
}

async fn add_specialties(
    abilities_diff: &AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    let ability_name_with_specialty_to_add: Vec<AbilityNameVanillaPostgres> = abilities_diff
        .specialties_to_add
        .iter()
        .map(|(ability_name, _)| (*ability_name).into())
        .collect();

    let specialty_name_to_add: Vec<&str> = abilities_diff
        .specialties_to_add
        .iter()
        .map(|(_, text)| text.as_str())
        .collect();

    query!(
        r#"
        INSERT INTO specialties
        SELECT
            $1::INTEGER as character_id,
            data.name as name,
            data.specialty as specialty
        FROM UNNEST($2::ABILITYNAMEVANILLA[], $3::VARCHAR(255)[]) AS data(name, specialty)
        "#,
        character_id as i32,
        &ability_name_with_specialty_to_add as &[AbilityNameVanillaPostgres],
        &specialty_name_to_add as &[&str],
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error attempting to insert specialties")?;

    Ok(())
}

pub async fn update_abilities(
    abilities_diff: AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if !abilities_diff.abilities_to_modify.is_empty() {
        update_vanilla_abilities(&abilities_diff, transaction, character_id)
            .await
            .wrap_err("Error attempting to upsert abilities")?;
    }

    if !abilities_diff.specialties_to_remove.is_empty() {
        remove_specialties(&abilities_diff, transaction, character_id)
            .await
            .wrap_err("Error attempting to remove specialties")?;
    }

    if !abilities_diff.specialties_to_add.is_empty() {
        add_specialties(&abilities_diff, transaction, character_id)
            .await
            .wrap_err("Error attempting to add specialties")?;
    }

    Ok(())
}
