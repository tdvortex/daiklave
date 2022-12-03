use crate::character::{
    create_character,
    traits::{experience::ExperiencePoints, willpower::Willpower, player::Player, campaign::Campaign},
    Character, CharacterBuilder,
};

use self::rows::{
    AbilityRow, ArmorRow, ArmorWornRow, AttributeRow, CampaignRow, CharacterRow, HealthBoxRow,
    IntimacyRow, MeritPrerequisiteSetRow, MeritRow, PlayerRow, PrerequisiteRow, SpecialtyRow,
    WeaponEquippedRow, WeaponRow,
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

pub async fn get_character(pool: &PgPool, character_id: i32) -> Result<Option<GetCharacter>> {
    Ok(
        sqlx::query_file_as!(GetCharacter, "src/database/get_character.sql", character_id)
            .fetch_optional(pool)
            .await?,
    )
}

impl CharacterBuilder {
    fn apply_player_row(&mut self, player_row: PlayerRow) -> &mut Self {
        self.with_player(Player::new(player_row.id, player_row.name))
    }

    fn apply_campaign_row(&mut self, campaign_row: CampaignRow) -> &mut Self {
        self.with_campaign(Campaign::new(campaign_row.id, campaign_row.name, campaign_row.bot_channel, campaign_row.description))
    }

    fn apply_character_row(&mut self, character_row: CharacterRow) -> Result<&mut Self> {
        let willpower = Willpower {
            current: character_row.current_willpower.try_into()?,
            maximum: character_row.max_willpower.try_into()?,
        };

        let experience = ExperiencePoints {
            current: character_row.current_experience.try_into()?,
            total: character_row.total_experience.try_into()?,
        };

        let applied = self
            .with_id(character_row.id)
            .with_name(character_row.name)
            .with_willpower(willpower)
            .with_experience(experience);

        // TODO: handle Exalt Type

        Ok(applied)
    }
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        let mut character = create_character();
        character.apply_player_row(self.player);
        if let Some(campaign) = self.campaign {
            character.apply_campaign_row(campaign);
        }
        character.apply_character_row(self.character)?;
        character.build()
    }
}
