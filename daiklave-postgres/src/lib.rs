use abilities::{AbilityRow, SpecialtyRow, apply_abilities_and_specialties_rows};
use armor::{ArmorRow, ArmorTagRow, ArmorWornRow, apply_armor_rows};
use attributes::{AttributeRow, apply_attribute_rows};
use campaign::{apply_campaign_row, CampaignRow};
use character::{apply_character_row, CharacterRow};
use craft::{CraftAbilityRow, CraftAbilitySpecialtyRow, apply_craft};
use daiklave_core::{
    id::Id,
    player::Player,
    Character,
};
use eyre::{Result, WrapErr};
use health::{HealthBoxRow, apply_health_box_rows};
use intimacies::{IntimacyRow, apply_intimacy_rows};
use martial_arts::{MartialArtsStyleRow, CharacterMartialArtsRow, CharacterMartialArtsSpecialtyRow, MartialArtsCharmRow, MartialArtsCharmKeywordRow, MartialArtsCharmCostRow, MartialArtsCharmTreeRow, apply_martial_arts};
use merits::{MeritTemplateRow, apply_merits_rows, MeritDetailRow, MeritPrerequisiteSetRow, PrerequisiteRow};
use sqlx::{query, PgPool, Postgres, Transaction};
use weapons::{WeaponRow, WeaponTagRow, WeaponEquippedRow, apply_weapon_rows};
mod abilities;
mod armor;
mod attributes;
mod campaign;
mod character;
mod craft;
mod health;
mod intimacies;
mod martial_arts;
mod merits;
mod weapons;

pub async fn destroy_character(pool: &PgPool, id: i32) -> Result<()> {
    query!(
        "DELETE FROM characters
        WHERE id = $1",
        id as i32
    )
    .execute(pool)
    .await
    .wrap_err_with(|| format!("Database error deleting character {}", id))?;

    Ok(())
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "players")]
pub struct PlayerRow {
    pub id: i32,
    pub name: String,
}

impl From<PlayerRow> for Player {
    fn from(row: PlayerRow) -> Self {
        Player {
            id: Id::Database(row.id),
            name: row.name,
        }
    }
}


















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
    martial_arts_styles: Option<Vec<MartialArtsStyleRow>>,
    character_martial_arts_styles: Option<Vec<CharacterMartialArtsRow>>,
    martial_arts_specialties: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
    martial_arts_charms: Option<Vec<MartialArtsCharmRow>>,
    martial_arts_charm_keywords: Option<Vec<MartialArtsCharmKeywordRow>>,
    martial_arts_charms_costs: Option<Vec<MartialArtsCharmCostRow>>,
    martial_arts_charm_tree: Option<Vec<MartialArtsCharmTreeRow>>,
    craft_abilities: Option<Vec<CraftAbilityRow>>,
    craft_specialties: Option<Vec<CraftAbilitySpecialtyRow>>,
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

async fn retrieve_character_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<Option<Character>> {
    let maybe_get_character = sqlx::query_file_as!(GetCharacter, "src/retrieve.sql", character_id)
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

struct AllMartialArtsRows {
    pub style_rows: Option<Vec<MartialArtsStyleRow>>,
    pub character_style_rows: Option<Vec<CharacterMartialArtsRow>>,
    pub specialty_rows: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
    pub martial_arts_charm_rows: Option<Vec<MartialArtsCharmRow>>,
    pub charm_keyword_rows: Option<Vec<MartialArtsCharmKeywordRow>>,
    pub charm_cost_rows: Option<Vec<MartialArtsCharmCostRow>>,
    pub charm_tree_rows: Option<Vec<MartialArtsCharmTreeRow>>,
}

impl TryInto<Character> for GetCharacter {
    type Error = eyre::Report;

    fn try_into(self) -> Result<Character, Self::Error> {
        let mut builder = Character::builder(self.character.id, self.player.into());
        builder = apply_campaign_row(builder, self.campaign);
        builder = apply_character_row(builder, self.character)
            .wrap_err("Could not apply character row")?;
        builder = apply_attribute_rows(builder, self.attributes)
            .wrap_err("Could not apply attribute rows")?;
        builder = apply_abilities_and_specialties_rows(builder, self.abilities, self.specialties)
            .wrap_err("Could not apply ability and specialty rows")?;
        builder = apply_craft(builder, self.craft_abilities, self.craft_specialties)
            .wrap_err("Could not apply craft rows")?;
        builder = apply_intimacy_rows(builder, self.intimacies);
        builder = apply_health_box_rows(builder, self.health_boxes);
        builder = apply_weapon_rows(builder, self.weapons_owned, self.weapon_tags, self.weapons_equipped)
            .wrap_err("Could not apply weapon rows")?;
        builder = apply_armor_rows(builder, self.armor_owned, self.armor_tags, self.armor_worn)
            .wrap_err("Could not apply armor rows")?;
        builder = apply_merits_rows(builder, 
                self.merit_templates,
                self.merit_details,
                self.merit_prerequisite_sets,
                self.merit_prerequisites,
            )
            .wrap_err("Could not apply merit rows")?;
        builder = apply_martial_arts(builder, AllMartialArtsRows {
                style_rows: self.martial_arts_styles,
                character_style_rows: self.character_martial_arts_styles,
                specialty_rows: self.martial_arts_specialties,
                martial_arts_charm_rows: self.martial_arts_charms,
                charm_keyword_rows: self.martial_arts_charm_keywords,
                charm_cost_rows: self.martial_arts_charms_costs,
                charm_tree_rows: self.martial_arts_charm_tree,
            })
            .wrap_err("Could not apply martial arts rows")?;

        builder.build()
    }
}

pub async fn update_character(pool: &PgPool, character: &Character) -> Result<Character> {
    todo!()
}
