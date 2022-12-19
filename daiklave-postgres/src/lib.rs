use abilities::{AbilityRow, SpecialtyRow, apply_abilities_and_specialties_rows};
use armor::{ArmorRow, ArmorTagRow, ArmorWornRow, apply_armor_rows};
use attributes::{AttributeRow, apply_attribute_rows};
use campaign::{apply_campaign_row, CampaignRow};
use character::{apply_character_row, CharacterRow};
use craft::{CraftAbilityRow, CraftAbilitySpecialtyRow, apply_craft};
use daiklave_core::{
    charms::{CharmActionType, CharmCostType, CharmKeyword},
    id::Id,
    player::Player,
    Character,
};
use eyre::{Result, WrapErr};
use health::{HealthBoxRow, apply_health_box_rows};
use intimacies::{IntimacyRow, apply_intimacy_rows};
use merits::{MeritTemplateRow, apply_merits_rows, MeritDetailRow, MeritPrerequisiteSetRow, PrerequisiteRow};
use sqlx::{postgres::PgHasArrayType, query, PgPool, Postgres, Transaction};
use weapons::{WeaponRow, WeaponTagRow, WeaponEquippedRow, apply_weapon_rows};
mod abilities;
mod armor;
mod attributes;
mod campaign;
mod character;
mod craft;
mod health;
mod intimacies;
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















#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "CHARMCOST")]
pub struct CharmCostPostgres {
    cost_type: CharmCostTypePostgres,
    amount: i16,
}

impl PgHasArrayType for CharmCostPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMCOST")
    }
}

impl From<CharmCostTypePostgres> for CharmCostType {
    fn from(value: CharmCostTypePostgres) -> Self {
        match value {
            CharmCostTypePostgres::Motes => Self::Motes,
            CharmCostTypePostgres::SorcerousMotes => Self::SorcerousMotes,
            CharmCostTypePostgres::Willpower => Self::Willpower,
            CharmCostTypePostgres::BashingHealth => Self::BashingHealth,
            CharmCostTypePostgres::LethalHealth => Self::LethalHealth,
            CharmCostTypePostgres::AggravatedHealth => Self::AggravatedHealth,
            CharmCostTypePostgres::AnimaLevels => Self::AnimaLevels,
            CharmCostTypePostgres::Initiative => Self::Initiative,
            CharmCostTypePostgres::Experience => Self::Experience,
            CharmCostTypePostgres::SilverCraftExperience => Self::SilverCraftExperience,
            CharmCostTypePostgres::GoldCraftExperience => Self::GoldCraftExperience,
            CharmCostTypePostgres::WhiteCraftExperience => Self::WhiteCraftExperience,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMKEYWORD", rename_all = "UPPERCASE")]
pub enum CharmKeywordPostgres {
    Air,
    Aggravated,
    Archetype,
    Aura,
    Balanced,
    Bridge,
    Clash,
    Counterattack,
    DecisiveOnly,
    Dual,
    Excellency,
    Fire,
    Earth,
    Mute,
    Pilot,
    Protean,
    Psyche,
    Perilous,
    Ritual,
    Salient,
    Signature,
    Stackable,
    Uniform,
    Water,
    WitheringOnly,
    Wood,
    WrittenOnly,
}

impl PgHasArrayType for CharmKeywordPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMKEYWORD")
    }
}

impl From<CharmKeywordPostgres> for CharmKeyword {
    fn from(keyword_pg: CharmKeywordPostgres) -> Self {
        match keyword_pg {
            CharmKeywordPostgres::Air => Self::Air,
            CharmKeywordPostgres::Aggravated => Self::Aggravated,
            CharmKeywordPostgres::Archetype => Self::Archetype,
            CharmKeywordPostgres::Aura => Self::Aura,
            CharmKeywordPostgres::Balanced => Self::Balanced,
            CharmKeywordPostgres::Bridge => Self::Bridge,
            CharmKeywordPostgres::Clash => Self::Clash,
            CharmKeywordPostgres::Counterattack => Self::Counterattack,
            CharmKeywordPostgres::DecisiveOnly => Self::DecisiveOnly,
            CharmKeywordPostgres::Dual => Self::Dual,
            CharmKeywordPostgres::Excellency => Self::Excellency,
            CharmKeywordPostgres::Fire => Self::Fire,
            CharmKeywordPostgres::Earth => Self::Earth,
            CharmKeywordPostgres::Mute => Self::Mute,
            CharmKeywordPostgres::Pilot => Self::Pilot,
            CharmKeywordPostgres::Protean => Self::Protean,
            CharmKeywordPostgres::Psyche => Self::Psyche,
            CharmKeywordPostgres::Perilous => Self::Perilous,
            CharmKeywordPostgres::Ritual => Self::Ritual,
            CharmKeywordPostgres::Salient => Self::Salient,
            CharmKeywordPostgres::Signature => Self::Signature,
            CharmKeywordPostgres::Stackable => Self::Stackable,
            CharmKeywordPostgres::Uniform => Self::Uniform,
            CharmKeywordPostgres::Water => Self::Water,
            CharmKeywordPostgres::WitheringOnly => Self::WitheringOnly,
            CharmKeywordPostgres::Wood => Self::Wood,
            CharmKeywordPostgres::WrittenOnly => Self::WrittenOnly,
        }
    }
}

impl From<CharmKeyword> for CharmKeywordPostgres {
    fn from(keyword: CharmKeyword) -> Self {
        match keyword {
            CharmKeyword::Air => Self::Air,
            CharmKeyword::Aggravated => Self::Aggravated,
            CharmKeyword::Archetype => Self::Archetype,
            CharmKeyword::Aura => Self::Aura,
            CharmKeyword::Balanced => Self::Balanced,
            CharmKeyword::Bridge => Self::Bridge,
            CharmKeyword::Clash => Self::Clash,
            CharmKeyword::Counterattack => Self::Counterattack,
            CharmKeyword::DecisiveOnly => Self::DecisiveOnly,
            CharmKeyword::Dual => Self::Dual,
            CharmKeyword::Excellency => Self::Excellency,
            CharmKeyword::Fire => Self::Fire,
            CharmKeyword::Earth => Self::Earth,
            CharmKeyword::Mute => Self::Mute,
            CharmKeyword::Pilot => Self::Pilot,
            CharmKeyword::Protean => Self::Protean,
            CharmKeyword::Psyche => Self::Psyche,
            CharmKeyword::Perilous => Self::Perilous,
            CharmKeyword::Ritual => Self::Ritual,
            CharmKeyword::Salient => Self::Salient,
            CharmKeyword::Signature => Self::Signature,
            CharmKeyword::Stackable => Self::Stackable,
            CharmKeyword::Uniform => Self::Uniform,
            CharmKeyword::Water => Self::Water,
            CharmKeyword::WitheringOnly => Self::WitheringOnly,
            CharmKeyword::Wood => Self::Wood,
            CharmKeyword::WrittenOnly => Self::WrittenOnly,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMACTIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmActionTypePostgres {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

impl From<CharmActionTypePostgres> for CharmActionType {
    fn from(value: CharmActionTypePostgres) -> Self {
        match value {
            CharmActionTypePostgres::Simple => Self::Simple,
            CharmActionTypePostgres::Supplemental => Self::Supplemental,
            CharmActionTypePostgres::Reflexive => Self::Reflexive,
            CharmActionTypePostgres::Permanent => Self::Permanent,
        }
    }
}

impl From<CharmActionType> for CharmActionTypePostgres {
    fn from(value: CharmActionType) -> Self {
        match value {
            CharmActionType::Simple => Self::Simple,
            CharmActionType::Supplemental => Self::Supplemental,
            CharmActionType::Reflexive => Self::Reflexive,
            CharmActionType::Permanent => Self::Permanent,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMCOSTTYPE", rename_all = "UPPERCASE")]
pub enum CharmCostTypePostgres {
    Motes,
    SorcerousMotes,
    Willpower,
    BashingHealth,
    LethalHealth,
    AggravatedHealth,
    AnimaLevels,
    Initiative,
    Experience,
    SilverCraftExperience,
    GoldCraftExperience,
    WhiteCraftExperience,
}

impl PgHasArrayType for CharmCostTypePostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMCOSTTYPE")
    }
}

impl From<CharmCostType> for CharmCostTypePostgres {
    fn from(value: CharmCostType) -> Self {
        match value {
            CharmCostType::Motes => Self::Motes,
            CharmCostType::SorcerousMotes => Self::SorcerousMotes,
            CharmCostType::Willpower => Self::Willpower,
            CharmCostType::BashingHealth => Self::BashingHealth,
            CharmCostType::LethalHealth => Self::LethalHealth,
            CharmCostType::AggravatedHealth => Self::AggravatedHealth,
            CharmCostType::AnimaLevels => Self::AnimaLevels,
            CharmCostType::Initiative => Self::Initiative,
            CharmCostType::Experience => Self::Experience,
            CharmCostType::SilverCraftExperience => Self::SilverCraftExperience,
            CharmCostType::GoldCraftExperience => Self::GoldCraftExperience,
            CharmCostType::WhiteCraftExperience => Self::WhiteCraftExperience,
        }
    }
}

#[derive(Debug)]
pub(crate) struct MartialArtsStyleRow {
    id: i32,
    name: String,
    description: String,
    book_title: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for MartialArtsStyleRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("martial_arts_styles")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MartialArtsStyleRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let description = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            description,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "character_martial_arts")]
pub(crate) struct CharacterMartialArtsRow {
    character_id: i32,
    style_id: i32,
    dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "character_martial_arts_specialties")]
pub(crate) struct CharacterMartialArtsSpecialtyRow {
    character_id: i32,
    style_id: i32,
    specialty: String,
}
#[derive(Debug)]
pub(crate) struct MartialArtsCharmRow {
    id: i32,
    style_id: i32,
    ability_dots_required: i16,
    essence_dots_required: i16,
    name: String,
    summary: Option<String>,
    description: String,
    action_type: CharmActionTypePostgres,
    duration: String,
    book_title: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for MartialArtsCharmRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("martial_arts_charms")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MartialArtsCharmRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let style_id = decoder.try_decode::<i32>()?;
        let ability_dots_required = decoder.try_decode::<i16>()?;
        let essence_dots_required = decoder.try_decode::<i16>()?;
        let name = decoder.try_decode::<String>()?;
        let summary = decoder.try_decode::<Option<String>>()?;
        let description = decoder.try_decode::<String>()?;
        let action_type = decoder.try_decode::<CharmActionTypePostgres>()?;
        let duration = decoder.try_decode::<String>()?;
        let book_title = decoder.try_decode::<Option<String>>()?;
        let page_number = decoder.try_decode::<Option<i16>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            style_id,
            ability_dots_required,
            essence_dots_required,
            name,
            summary,
            description,
            action_type,
            duration,
            book_title,
            page_number,
            creator_id,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charms_keywords")]
pub(crate) struct MartialArtsCharmKeywordRow {
    charm_id: i32,
    charm_keyword: CharmKeywordPostgres,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charms_costs")]
pub(crate) struct MartialArtsCharmCostRow {
    charm_id: i32,
    cost: CharmCostTypePostgres,
    amount: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charm_tree")]
pub(crate) struct MartialArtsCharmTreeRow {
    child_id: i32,
    parent_id: i32,
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
