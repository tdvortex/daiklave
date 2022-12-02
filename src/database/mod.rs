use self::rows::{
    AbilityRow, AttributeRow, CampaignRow, CharacterRow, HealthBoxRow, IntimacyRow, PlayerRow,
    SpecialtyRow,
};
use eyre::Result;
use sqlx::PgPool;

pub mod enums;
pub mod queries;
pub mod rows;

#[derive(Debug)]
pub struct GetCharacter {
    pub character: CharacterRow,
    pub player: PlayerRow,
    pub campaign: CampaignRow,
    pub attributes: Vec<AttributeRow>,
    pub abilities: Vec<AbilityRow>,
    pub specialties: Option<Vec<SpecialtyRow>>,
    pub intimacies: Option<Vec<IntimacyRow>>,
    pub health_boxes: Vec<HealthBoxRow>,
}

impl GetCharacter {
    pub async fn execute(pool: &PgPool, character_id: i64) -> Result<Option<GetCharacter>> {
        Ok(
            sqlx::query_file_as!(GetCharacter, "src/database/get_character.sql", character_id)
                .fetch_optional(pool)
                .await?,
        )
    }
}
