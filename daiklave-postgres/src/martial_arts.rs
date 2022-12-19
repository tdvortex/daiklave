use std::collections::HashMap;

use daiklave_core::{charms::{CharmCostType, CharmKeyword, CharmActionType}, character::CharacterBuilder, martial_arts::{MartialArtsStyle, MartialArtsCharmBuilder, MartialArtsCharm}, id::Id};
use sqlx::postgres::PgHasArrayType;
use eyre::{eyre, Result, WrapErr};
use crate::AllMartialArtsRows;

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

pub fn apply_martial_arts(mut builder: CharacterBuilder, all_rows: AllMartialArtsRows) -> Result<CharacterBuilder> {
    if all_rows.character_style_rows.is_none() {
        return Ok(builder);
    }

    if all_rows.style_rows.is_none() {
        return Err(eyre!("No styles available to apply to character"));
    }

    // Construct styles from style rows, leave space for specialties
    let mut style_map = all_rows.style_rows.unwrap().into_iter().fold(
        Ok(HashMap::new()),
        |result_map, row| {
            result_map.and_then(|mut map| {
                let builder = if row.book_title.is_some()
                    && row.page_number.is_some()
                    && row.creator_id.is_none()
                {
                    MartialArtsStyle::from_book(
                        Id::Database(row.id),
                        row.book_title.unwrap(),
                        row.page_number.unwrap(),
                    )
                } else if row.book_title.is_none()
                    && row.page_number.is_none()
                    && row.creator_id.is_some()
                {
                    MartialArtsStyle::custom(
                        Id::Database(row.id),
                        Id::Database(row.creator_id.unwrap()),
                    )
                } else {
                    return Err(eyre!(
                        "Database error: inconsistent data source for martial arts style {}",
                        row.id
                    ));
                };

                let style = builder
                    .with_name(row.name)
                    .with_description(row.description)
                    .build()?;

                map.insert(style.id(), (style, Vec::new()));
                Ok(map)
            })
        },
    )?;

    // Construct character's specialties for styles
    if let Some(rows) = all_rows.specialty_rows {
        for row in rows.into_iter() {
            if let Some(ptr) = style_map.get_mut(&Id::Database(row.style_id)) {
                ptr.1.push(row.specialty);
            } else {
                return Err(eyre!("Style {} not found", row.style_id));
            }
        }
    }

    // Apply styles and specialties to character
    builder = all_rows.character_style_rows.unwrap().into_iter().fold(
        Ok(builder),
        |result_self, row| {
            let (style, specialties) = style_map
                .remove(&Id::Database(row.style_id))
                .ok_or_else(|| eyre!("Style {} not found", row.style_id))?;
            let style_id = style.id();
            let dots: u8 = row
                .dots
                .try_into()
                .wrap_err_with(|| format!("Invalid number of dots: {}", row.dots))?;

            result_self.and_then(|builder| {
                specialties.into_iter().fold(
                    builder.with_martial_arts_style(style, dots),
                    |res_b, specialty| {
                        res_b.and_then(|b| b.with_martial_arts_specialty(style_id, specialty))
                    },
                )
            })
        },
    )?;

    if all_rows.martial_arts_charm_rows.is_none() {
        // No charms to build or apply
        return Ok(builder);
    }

    // Construct charms except for keywords and costs
    let mut charm_builder_map: HashMap<i32, MartialArtsCharmBuilder>;
    if let Some(rows) = all_rows.martial_arts_charm_rows {
        charm_builder_map = HashMap::new();
        for row in rows.into_iter() {
            let mut builder = if row.book_title.is_some()
                && row.page_number.is_some()
                && row.creator_id.is_none()
            {
                MartialArtsCharm::from_book(
                    Id::Database(row.id),
                    row.book_title.unwrap(),
                    row.page_number.unwrap(),
                )
            } else if row.book_title.is_none()
                && row.page_number.is_none()
                && row.creator_id.is_some()
            {
                MartialArtsCharm::custom(
                    Id::Database(row.id),
                    Id::Database(row.creator_id.unwrap()),
                )
            } else {
                return Err(eyre!(
                    "Database error: inconsistent data source for martial arts charm {}",
                    row.id
                ));
            };
            let martial_arts_dots =
                row.ability_dots_required.try_into().wrap_err_with(|| {
                    format!(
                        "Invalid number of martial arts dots: {}",
                        row.ability_dots_required
                    )
                })?;
            let essence_rating = row.essence_dots_required.try_into().wrap_err_with(|| {
                format!("Invalid essence requirement: {}", row.essence_dots_required)
            })?;

            builder = builder
                .for_martial_arts_style(Id::Database(row.style_id))
                .with_name(row.name)
                .with_action_type(row.action_type.into())
                .with_description(row.description)
                .with_duration(row.duration)
                .requiring_martial_arts_dots(martial_arts_dots)
                .requiring_essence(essence_rating);

            if let Some(summary) = row.summary {
                builder = builder.with_summary(summary);
            }

            charm_builder_map.insert(row.id, builder);
        }
    } else {
        return Ok(builder);
    }

    // Group charm keywords
    let mut charm_keyword_map: HashMap<Id, Vec<CharmKeyword>> = HashMap::new();
    if let Some(rows) = all_rows.charm_keyword_rows {
        for row in rows.into_iter() {
            let id = Id::Database(row.charm_id);
            charm_keyword_map
                .entry(id)
                .or_default()
                .push(row.charm_keyword.into());
        }
    }

    // Group charm costs
    let mut charm_costs_map: HashMap<Id, Vec<(CharmCostType, u8)>> = HashMap::new();
    if let Some(rows) = all_rows.charm_cost_rows {
        for row in rows.into_iter() {
            let id = Id::Database(row.charm_id);
            charm_costs_map.entry(id).or_default().push((
                row.cost.into(),
                row.amount
                    .try_into()
                    .wrap_err_with(|| format!("Invalid cost amount: {}", row.amount))?,
            ));
        }
    }

    // Group charm prerequisites
    let mut charm_prerequisites_map: HashMap<Id, Vec<Id>> = HashMap::new();
    if let Some(rows) = all_rows.charm_tree_rows {
        for row in rows.into_iter() {
            let child_id = Id::Database(row.child_id);
            let parent_id = Id::Database(row.parent_id);

            charm_prerequisites_map
                .entry(child_id)
                .or_default()
                .push(parent_id);
        }
    }

    for (charm_id, mut charm_builder) in charm_builder_map.into_iter() {
        if let Some(keywords) = charm_keyword_map.remove(&Id::Database(charm_id)) {
            for keyword in keywords.into_iter() {
                charm_builder = charm_builder.with_keyword(keyword);
            }
        }

        if let Some(costs) = charm_costs_map.remove(&Id::Database(charm_id)) {
            for (cost_type, amount) in costs.into_iter() {
                charm_builder = charm_builder.with_cost(cost_type, amount);
            }
        }

        if let Some(prerequisites) = charm_prerequisites_map.remove(&Id::Database(charm_id)) {
            for prerequisite_id in prerequisites.into_iter() {
                charm_builder = charm_builder.with_charm_prerequisite(prerequisite_id);
            }
        }

        let charm = charm_builder.build()?;

        builder = builder.with_martial_arts_charm(charm)?;
    }

    Ok(builder)
}