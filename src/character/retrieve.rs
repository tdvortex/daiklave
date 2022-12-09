use eyre::Result;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    abilities::tables::{AbilityRow, SpecialtyRow},
    armor::tables::{ArmorRow, ArmorWornRow},
    attributes::tables::AttributeRow,
    campaign::tables::CampaignRow,
    character::Character,
    character::tables::CharacterRow,
    health::tables::HealthBoxRow,
    intimacies::tables::IntimacyRow,
    merits::tables::{MeritDetailRow, MeritPrerequisiteSetRow, MeritTemplateRow},
    player::tables::PlayerRow,
    prerequisite::tables::PrerequisiteRow,
    weapons::tables::{WeaponEquippedRow, WeaponRow},
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
    weapons_owned: Vec<WeaponRow>,
    weapons_equipped: Option<Vec<WeaponEquippedRow>>,
    armor_owned: Option<Vec<ArmorRow>>,
    armor_worn: Option<Vec<ArmorWornRow>>,
    merit_templates: Option<Vec<MeritTemplateRow>>,
    merit_details: Option<Vec<MeritDetailRow>>,
    merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    merit_prerequisites: Option<Vec<PrerequisiteRow>>,
}

pub async fn retrieve_character(pool: &PgPool, character_id: i32) -> Result<Option<Character>> {
    let mut transaction = pool.begin().await?;

    let maybe_character = retrieve_character_transaction(&mut transaction, character_id).await?;

    transaction.commit().await?;
    Ok(maybe_character)
}

pub(crate) async fn retrieve_character_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<Option<Character>> {
    let maybe_get_character = sqlx::query_file_as!(
        GetCharacter,
        "src/character/retrieve.sql",
        character_id
    )
    .fetch_optional(&mut *transaction)
    .await?;

    if let Some(get_character) = maybe_get_character {
        Ok(Some(get_character.try_into()?))
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
            .apply_character_row(self.character)?
            .apply_attribute_rows(self.attributes)?
            .apply_abilities_and_specialties_rows(self.abilities, self.specialties)?
            .apply_intimacy_rows(self.intimacies)
            .apply_health_box_rows(self.health_boxes)
            .apply_weapon_rows(self.weapons_owned, self.weapons_equipped)?
            .apply_armor_rows(self.armor_owned, self.armor_worn)?
            .apply_merits_rows(
                self.merit_templates,
                self.merit_details,
                self.merit_prerequisite_sets,
                self.merit_prerequisites,
            )?
            .build()
    }
}
