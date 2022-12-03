use crate::character::{
    builder::create_character,
    builder::CharacterBuilder,
    traits::{
        campaign::Campaign, experience::ExperiencePoints, player::Player, willpower::Willpower,
    },
    Character,
};

use self::{
    enums::AbilityName,
    rows::{
        AbilityRow, ArmorRow, ArmorWornRow, AttributeRow, CampaignRow, CharacterRow, HealthBoxRow,
        IntimacyRow, MeritPrerequisiteSetRow, MeritRow, PlayerRow, PrerequisiteRow, SpecialtyRow,
        WeaponEquippedRow, WeaponRow,
    },
};
use eyre::{eyre, Result};
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
        self.with_campaign(Campaign::new(
            campaign_row.id,
            campaign_row.name,
            campaign_row.bot_channel,
            campaign_row.description,
        ))
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

        Ok(applied)
    }

    fn apply_attribute_row(&mut self, attribute_row: AttributeRow) -> Result<&mut Self> {
        let attribute_name = attribute_row.name.into();
        let value = attribute_row.dots.try_into()?;

        self.with_attribute(attribute_name, value)
    }

    fn apply_ability_row(&mut self, ability_row: AbilityRow) -> Result<&mut Self> {
        let dots: u8 = ability_row.dots.try_into()?;

        if ability_row.name == AbilityName::Craft {
            let craft_focus = ability_row
                .subskill
                .ok_or(eyre!("craft abilities must have a focus"))?;
            Ok(self.with_craft(craft_focus, dots))
        } else if ability_row.name == AbilityName::MartialArts {
            let martial_arts_style = ability_row
                .subskill
                .ok_or(eyre!("martial arts abilities must have a style"))?;
            Ok(self.with_martial_arts(martial_arts_style, dots))
        } else {
            Ok(self.with_ability(ability_row.name.try_into().unwrap(), dots))
        }
    }
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        let mut character = create_character();
        character.apply_player_row(self.player);

        self.campaign
            .map(|campaign| character.apply_campaign_row(campaign));

        character.apply_character_row(self.character)?;

        self.attributes.into_iter().fold(
            Ok(&mut character),
            |character_result, attribute_row| {
                character_result.and_then(|character| character.apply_attribute_row(attribute_row))
            },
        )?;

        self.abilities
            .into_iter()
            .fold(Ok(&mut character), |character_result, ability_row| {
                character_result.and_then(|character| character.apply_ability_row(ability_row))
            })?;

        character.build()
    }
}
