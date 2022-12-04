use eyre::Result;
use sqlx::PgPool;

use crate::{
    character::{builder::create_character, Character},
    database::tables::{
        abilities::{AbilityRow, SpecialtyRow},
        armor::{ArmorRow, ArmorWornRow},
        attributes::AttributeRow,
        campaign::CampaignRow,
        character::CharacterRow,
        health::HealthBoxRow,
        intimacies::IntimacyRow,
        merits::{MeritDetailRow, MeritPrerequisiteSetRow, MeritTemplateRow},
        players::PlayerRow,
        prerequisites::PrerequisiteRow,
        weapons::{WeaponEquippedRow, WeaponRow},
    },
};

#[derive(Debug)]
pub struct GetCharacter {
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
    pub merit_templates: Option<Vec<MeritTemplateRow>>,
    pub merit_details: Option<Vec<MeritDetailRow>>,
    pub merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    pub merit_prerequisites: Option<Vec<PrerequisiteRow>>,
}

pub async fn get_character(pool: &PgPool, character_id: i32) -> Result<Option<GetCharacter>> {
    Ok(sqlx::query_file_as!(
        GetCharacter,
        "src/database/queries/get_character.sql",
        character_id
    )
    .fetch_optional(pool)
    .await?)
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        let mut character = create_character();
        character.apply_player_row(self.player);
        character.apply_campaign_row(self.campaign);
        character.apply_character_row(self.character)?;
        character.apply_attribute_rows(self.attributes)?;
        character.apply_abilities_and_specialties_rows(self.abilities, self.specialties)?;
        character.apply_intimacy_rows(self.intimacies);
        character.apply_health_box_rows(self.health_boxes);
        character.apply_weapon_rows(self.weapons_owned, self.weapons_equipped)?;
        character.apply_armor_rows(self.armor_owned, self.armor_worn)?;

        character.build()
    }
}
