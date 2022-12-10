use eyre::{Result, WrapErr};
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    abilities::tables::{AbilityRow, SpecialtyRow},
    armor::tables::{ArmorRow, ArmorTagRow, ArmorWornRow},
    attributes::tables::AttributeRow,
    campaign::tables::CampaignRow,
    character::tables::CharacterRow,
    character::Character,
    health::tables::HealthBoxRow,
    intimacies::tables::IntimacyRow,
    merits::tables::{MeritDetailRow, MeritPrerequisiteSetRow, MeritTemplateRow},
    player::tables::PlayerRow,
    prerequisite::tables::PrerequisiteRow,
    weapons::tables::{WeaponEquippedRow, WeaponRow, WeaponTagRow},
};

#[derive(Debug)]
struct GetCharacter {
    character: CharacterRow,
    player: PlayerRow,
    campaign: Option<CampaignRow>,
    attributes: Vec<AttributeRow>,
    abilities: Vec<AbilityRow>,
    specialties: Option<Vec<SpecialtyRow>>,
    intimacies: Option<Vec<IntimacyRow>>,
    health_boxes: Vec<HealthBoxRow>,
    weapons_owned: Option<Vec<WeaponRow>>,
    weapon_tags: Option<Vec<WeaponTagRow>>,
    weapons_equipped: Option<Vec<WeaponEquippedRow>>,
    armor_owned: Option<Vec<ArmorRow>>,
    armor_tags: Option<Vec<ArmorTagRow>>,
    armor_worn: Option<Vec<ArmorWornRow>>,
    merit_templates: Option<Vec<MeritTemplateRow>>,
    merit_details: Option<Vec<MeritDetailRow>>,
    merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    merit_prerequisites: Option<Vec<PrerequisiteRow>>,
}

pub async fn retrieve_character(pool: &PgPool, character_id: i32) -> Result<Option<Character>> {
    let mut transaction = pool
        .begin()
        .await
        .wrap_err("Error attempting to start transaction")?;

    let maybe_character = retrieve_character_transaction(&mut transaction, character_id).await?;

    transaction
        .commit()
        .await
        .wrap_err("Error attempting to commit transaction")?;
    Ok(maybe_character)
}

pub(crate) async fn retrieve_character_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<Option<Character>> {
    let maybe_get_character =
        sqlx::query_file_as!(GetCharacter, "src/character/retrieve.sql", character_id)
            .fetch_optional(&mut *transaction)
            .await
            .wrap_err_with(|| {
                format!(
                    "Database error trying to retrieve character with id: {}",
                    character_id
                )
            })?;

    if let Some(get_character) = maybe_get_character {
        Ok(Some(get_character.try_into().wrap_err_with(|| {
            format!(
                "Error attempting to convert database output to Character for character_id {}",
                character_id
            )
        })?))
    } else {
        // Valid query with no character
        Ok(None)
    }
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        Character::create()
            .apply_player_row(self.player)
            .apply_campaign_row(self.campaign)
            .apply_character_row(self.character)
            .wrap_err("Could not apply character row")?
            .apply_attribute_rows(self.attributes)
            .wrap_err("Could not apply attribute rows")?
            .apply_abilities_and_specialties_rows(self.abilities, self.specialties)
            .wrap_err("Could not apply ability and specialty rows")?
            .apply_intimacy_rows(self.intimacies)
            .apply_health_box_rows(self.health_boxes)
            .apply_weapon_rows(self.weapons_owned, self.weapon_tags, self.weapons_equipped)
            .wrap_err("Could not apply weapon rows")?
            .apply_armor_rows(self.armor_owned, self.armor_tags, self.armor_worn)
            .wrap_err("Could not apply armor rows")?
            .apply_merits_rows(
                self.merit_templates,
                self.merit_details,
                self.merit_prerequisite_sets,
                self.merit_prerequisites,
            )
            .wrap_err("Could not apply merit rows")?
            .build()
    }
}
