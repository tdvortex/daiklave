use self::rows::{
    AbilityRow, AttributeRow, CampaignRow, CharacterRow, HealthBoxRow, IntimacyRow, PlayerRow,
    SpecialtyRow, WeaponRow, ArmorRow, WeaponEquippedRow, ArmorWornRow, MeritRow, MeritPrerequisiteSetRow, PrerequisiteRow,
};
use eyre::Result;
use sqlx::PgPool;

pub mod composites;
pub mod enums;
pub mod rows;

#[derive(Debug)]
pub struct GetCharacter {
    pub character: CharacterRow,
    pub player: PlayerRow,
    pub campaign: Option<CampaignRow>,
    pub attributes: Vec<AttributeRow>,
    pub abilities: Vec<AbilityRow>,
    pub specialties: Option<Vec<SpecialtyRow>>,
    pub intimacies: Option<Vec<IntimacyRow>>,
    pub health_boxes: Vec<HealthBoxRow>,
    pub weapons_owned: Vec<WeaponRow>,
    pub weapons_equipped: Option<Vec<WeaponEquippedRow>>,
    pub armor_owned: Option<Vec<ArmorRow>>,
    pub armor_worn: Option<Vec<ArmorWornRow>>,
    pub merits: Option<Vec<MeritRow>>,
    pub merit_prerequisite_sets: Option<Vec<MeritPrerequisiteSetRow>>,
    pub merit_prerequisites: Option<Vec<PrerequisiteRow>>,
}

impl GetCharacter {
    pub async fn execute(pool: &PgPool, character_id: i32) -> Result<Option<GetCharacter>> {
        Ok(
            sqlx::query_file_as!(GetCharacter, "src/database/get_character.sql", character_id)
                .fetch_optional(pool)
                .await?,
        )
    }
}
