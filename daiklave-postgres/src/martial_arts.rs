use std::collections::HashMap;

use daiklave_core::{
    character::CharacterBuilder,
    charms::{CharmActionType, CharmCostType, CharmKeyword},
    data_source::DataSource,
    id::{Id, CharacterId, MartialArtsStyleId, MartialArtsCharmId},
    martial_arts::{
        diff::MartialArtsDiff, MartialArtsCharm, MartialArtsCharmBuilder, MartialArtsStyle,
    },
};
use eyre::{eyre, Result, WrapErr};
use sqlx::{postgres::PgHasArrayType, query, Postgres, Transaction};

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
pub struct MartialArtsStyleRow {
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
pub struct CharacterMartialArtsRow {
    character_id: i32,
    style_id: i32,
    dots: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "character_martial_arts_specialties")]
pub struct CharacterMartialArtsSpecialtyRow {
    character_id: i32,
    style_id: i32,
    specialty: String,
}
#[derive(Debug)]
pub struct MartialArtsCharmRow {
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
pub struct MartialArtsCharmKeywordRow {
    charm_id: i32,
    charm_keyword: CharmKeywordPostgres,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charms_costs")]
pub struct MartialArtsCharmCostRow {
    charm_id: i32,
    cost: CharmCostTypePostgres,
    amount: i16,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "martial_arts_charm_tree")]
pub struct MartialArtsCharmTreeRow {
    child_id: i32,
    parent_id: i32,
}

pub struct AllMartialArtsRows {
    pub style_rows: Option<Vec<MartialArtsStyleRow>>,
    pub character_style_rows: Option<Vec<CharacterMartialArtsRow>>,
    pub specialty_rows: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
    pub martial_arts_charm_rows: Option<Vec<MartialArtsCharmRow>>,
    pub charm_keyword_rows: Option<Vec<MartialArtsCharmKeywordRow>>,
    pub charm_cost_rows: Option<Vec<MartialArtsCharmCostRow>>,
    pub charm_tree_rows: Option<Vec<MartialArtsCharmTreeRow>>,
}

pub fn apply_martial_arts(
    mut builder: CharacterBuilder,
    all_rows: AllMartialArtsRows,
) -> Result<CharacterBuilder> {
    if all_rows.character_style_rows.is_none() {
        return Ok(builder);
    }

    if all_rows.style_rows.is_none() {
        return Err(eyre!("No styles available to apply to character"));
    }

    // Construct styles from style rows, leave space for specialties
    let mut style_map =
        all_rows
            .style_rows
            .unwrap()
            .into_iter()
            .fold(Ok(HashMap::new()), |result_map, row| {
                result_map.and_then(|mut map| {
                    let builder = if row.book_title.is_some()
                        && row.page_number.is_some()
                        && row.creator_id.is_none()
                    {
                        MartialArtsStyle::from_book(
                            MartialArtsStyleId(Id::Database(row.id)),
                            row.book_title.unwrap(),
                            row.page_number.unwrap(),
                        )
                    } else if row.book_title.is_none()
                        && row.page_number.is_none()
                        && row.creator_id.is_some()
                    {
                        MartialArtsStyle::custom(
                            MartialArtsStyleId(Id::Database(row.id)),
                            CharacterId(Id::Database(row.creator_id.unwrap())),
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
            })?;

    // Construct character's specialties for styles
    if let Some(rows) = all_rows.specialty_rows {
        for row in rows.into_iter() {
            if let Some(ptr) = style_map.get_mut(&MartialArtsStyleId(Id::Database(row.style_id))) {
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
                .remove(&MartialArtsStyleId(Id::Database(row.style_id)))
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
                    MartialArtsCharmId(Id::Database(row.id)),
                    row.book_title.unwrap(),
                    row.page_number.unwrap(),
                )
            } else if row.book_title.is_none()
                && row.page_number.is_none()
                && row.creator_id.is_some()
            {
                MartialArtsCharm::custom(
                    MartialArtsCharmId(Id::Database(row.id)),
                    CharacterId(Id::Database(row.creator_id.unwrap())),
                )
            } else {
                return Err(eyre!(
                    "Database error: inconsistent data source for martial arts charm {}",
                    row.id
                ));
            };
            let martial_arts_dots = row.ability_dots_required.try_into().wrap_err_with(|| {
                format!(
                    "Invalid number of martial arts dots: {}",
                    row.ability_dots_required
                )
            })?;
            let essence_rating = row.essence_dots_required.try_into().wrap_err_with(|| {
                format!("Invalid essence requirement: {}", row.essence_dots_required)
            })?;

            builder = builder
                .for_martial_arts_style(MartialArtsStyleId(Id::Database(row.style_id)))
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
    let mut charm_prerequisites_map: HashMap<MartialArtsCharmId, Vec<MartialArtsCharmId>> = HashMap::new();
    if let Some(rows) = all_rows.charm_tree_rows {
        for row in rows.into_iter() {
            let child_id = MartialArtsCharmId(Id::Database(row.child_id));
            let parent_id = MartialArtsCharmId(Id::Database(row.parent_id));
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

        if let Some(prerequisites) = charm_prerequisites_map.remove(&MartialArtsCharmId(Id::Database(charm_id))) {
            for prerequisite_id in prerequisites.into_iter() {
                charm_builder = charm_builder.with_charm_prerequisite(prerequisite_id);
            }
        }

        let charm = charm_builder.build()?;

        builder = builder.with_martial_arts_charm(charm)?;
    }

    Ok(builder)
}

async fn upsert_character_styles(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
    style_database_id: i32,
    style_dots: u8,
    maybe_specialties: Option<&Vec<String>>,
) -> Result<()> {
    query!(
        "INSERT INTO character_martial_arts(character_id, style_id, dots)
        VALUES ($1::INTEGER, $2::INTEGER, $3::SMALLINT)
        ON CONFLICT (character_id, style_id) DO UPDATE
        SET dots = EXCLUDED.dots;",
        character_id as i32,
        style_database_id as i32,
        style_dots as i16
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error upserting martial arts style dots from character {} for style {}",
            character_id, style_database_id
        )
    })?;

    query!(
        "DELETE FROM character_martial_arts_specialties
        WHERE character_id = $1 AND style_id = $2",
        character_id as i32,
        style_database_id as i32,
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error deleting martial arts specialties from character {} for style {}",
            character_id, style_database_id
        )
    })?;

    if let Some(specialties) = maybe_specialties {
        let unpacked: Vec<&str> = specialties.iter().map(|s| s.as_str()).collect();

        query!(
            "INSERT INTO character_martial_arts_specialties(character_id, style_id, specialty)
            SELECT
                $1::INTEGER as character_id,
                $2::INTEGER as style_id,
                data.specialty as specialty
            FROM UNNEST($3::VARCHAR(255)[]) as data(specialty)",
            character_id as i32,
            style_database_id as i32,
            &unpacked as &[&str]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err_with(|| {
            format!(
                "Database error inserting martial arts specialties from character {} for style {}",
                character_id, style_database_id
            )
        })?;
    }

    Ok(())
}

async fn upsert_character_charms(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
    style_charms: &[MartialArtsCharm],
) -> Result<()> {
    let mut charm_database_ids: Vec<i32> = Vec::new();

    let mut placeholder_to_database_map = HashMap::new();
    for charm in style_charms.iter() {
        if let MartialArtsCharmId(Id::Database(id)) = charm.id() {
            charm_database_ids.push(id);
        } else {
            let placeholder = charm.id();
            let id = if let DataSource::Custom(_) = charm.data_source() {
                create_martial_arts_charm_transaction(transaction, charm, Some(character_id))
                    .await
                    .wrap_err("Error attempting to create book referenced martial arts charm")?
            } else {
                create_martial_arts_charm_transaction(transaction, charm, None)
                    .await
                    .wrap_err("Error attempting to create custom martial arts charm")?
            };
            placeholder_to_database_map.insert(placeholder, Id::Database(id));
            charm_database_ids.push(id)
        }
    }

    let mut prerequisite_pairs = Vec::new();
    for charm in style_charms.iter() {
        if charm.id().is_placeholder() {
            let child_id = **placeholder_to_database_map
                .get(&charm.id())
                .ok_or_else(|| {
                    eyre!(
                        "Martial arts charm placeholder id {} not successfully inserted",
                        **charm.id()
                    )
                })?;

            for prerequisite_charm_id in charm.prerequisite_charm_ids().iter() {
                if let MartialArtsCharmId(Id::Database(id)) = prerequisite_charm_id {
                    prerequisite_pairs.push((child_id, *id));
                } else {
                    let parent_id = **placeholder_to_database_map.get(prerequisite_charm_id).ok_or_else(|| {
                        eyre!("Unknown martial arts charm placeholder id {}", ***prerequisite_charm_id)
                    })?;
                    prerequisite_pairs.push((child_id, parent_id));
                }
            }
        }
    }

    if !prerequisite_pairs.is_empty() {
        create_martial_arts_charm_tree(transaction, &prerequisite_pairs)
            .await
            .wrap_err("Error attempting to create martial arts charm prerequisites tree")?;
    }

    query!(
        "INSERT INTO character_martial_arts_charms(character_id, charm_id)
        SELECT
            $1::INTEGER as character_id,
            data.charm_id as charm_id
        FROM UNNEST($2::INTEGER[]) as data(charm_id)",
        character_id,
        &charm_database_ids as &[i32]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error adding martial arts charms for character {}",
            character_id
        )
    })?;

    Ok(())
}

async fn remove_character_styles(
    martial_arts_diff: &MartialArtsDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    let removed_ids: Vec<i32> = martial_arts_diff
        .removed_styles
        .iter()
        .filter_map(|id| {
            if !id.is_placeholder() {
                Some(***id)
            } else {
                None
            }
        })
        .collect();

    if !removed_ids.is_empty() {
        query!(
            "DELETE FROM character_martial_arts
            WHERE character_id = $1 AND style_id IN (SELECT data.style_id FROM UNNEST($2::INTEGER[]) as data(style_id))",
            character_id as i32,
            &removed_ids as &[i32]
        ).execute(&mut *transaction).await.wrap_err_with(|| format!("Database error removing martial arts styles from character {}", character_id))?;
    }
    Ok(())
}

async fn add_styles_to_character(
    martial_arts_diff: &MartialArtsDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    for (style, style_dots, maybe_specialties, style_charms) in
        martial_arts_diff.added_styles.iter()
    {
        let style_database_id = if let MartialArtsStyleId(Id::Database(id)) = style.id() {
            id
        } else if let DataSource::Custom(_) = style.data_source() {
            create_martial_arts_style_transaction(transaction, style, Some(character_id)).await?
        } else {
            create_martial_arts_style_transaction(transaction, style, None).await?
        };

        upsert_character_styles(
            transaction,
            character_id,
            style_database_id,
            *style_dots,
            maybe_specialties.as_ref(),
        )
        .await
        .wrap_err("Error attemping to upsert character styles")?;

        if !style_charms.is_empty() {
            upsert_character_charms(transaction, character_id, style_charms).await?;
        }
    }

    Ok(())
}

async fn update_character_styles(
    martial_arts_diff: &MartialArtsDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    for (style_id, style_dots, maybe_specialties, style_charms) in
        martial_arts_diff.modified_styles.iter()
    {
        if style_id.is_placeholder() {
            return Err(eyre!("Cannot update a style with a placeholder value"));
        }

        upsert_character_styles(
            transaction,
            character_id,
            ***style_id,
            *style_dots,
            maybe_specialties.as_ref(),
        )
        .await?;

        query!(
            "DELETE FROM character_martial_arts_charms
            WHERE character_id = $1",
            character_id
        )
        .execute(&mut *transaction)
        .await
        .wrap_err_with(|| {
            format!(
                "Database error removing martial arts charms for character {}",
                character_id
            )
        })?;

        if !style_charms.is_empty() {
            upsert_character_charms(transaction, character_id, style_charms).await?;
        }
    }

    Ok(())
}

pub async fn update_martial_arts(
    martial_arts_diff: MartialArtsDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    remove_character_styles(&martial_arts_diff, transaction, character_id)
        .await
        .wrap_err("Error removing character martial arts styles")?;
    add_styles_to_character(&martial_arts_diff, transaction, character_id)
        .await
        .wrap_err("Error adding character martial arts styles")?;
    update_character_styles(&martial_arts_diff, transaction, character_id)
        .await
        .wrap_err("Error updating character martial arts styles")?;
    Ok(())
}

pub(crate) async fn create_martial_arts_style_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    style: &MartialArtsStyle,
    creator_id: Option<i32>,
) -> Result<i32> {
    query!(
        "INSERT INTO martial_arts_styles(name, description, book_title, page_number, creator_id)
        VALUES($1::VARCHAR(255), $2::TEXT, $3::VARCHAR(255), $4::SMALLINT, $5::INTEGER)
        RETURNING id
        ",
        style.name(),
        style.description(),
        style.data_source().book_title(),
        style.data_source().page_number(),
        creator_id
    )
    .fetch_one(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error attempting to insert martial arts style {}",
            style.name()
        )
    })
    .map(|record| record.id)
}

pub(crate) async fn create_martial_arts_charm_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    charm: &MartialArtsCharm,
    creator_id: Option<i32>,
) -> Result<i32> {
    if !charm.id().is_placeholder() {
        return Ok(**charm.id());
    }

    let action_type_pg: CharmActionTypePostgres = charm.action_type().into();

    let charm_id = query!(
        "INSERT INTO martial_arts_charms (style_id, ability_dots_required, essence_dots_required, name, summary, description, action_type, duration, book_title, page_number, creator_id)
        VALUES ($1::INTEGER, $2::SMALLINT, $3::SMALLINT, $4::VARCHAR(255), $5::TEXT, $6::TEXT, $7::CHARMACTIONTYPE, $8::VARCHAR(255), $9::VARCHAR(255), $10::SMALLINT, $11::INTEGER)
        RETURNING id",
        **charm.style_id() as i32,
        charm.martial_arts_requirement() as i16,
        charm.essence_requirement() as i16,
        charm.name() as &str,
        charm.summary() as Option<&str>,
        charm.description() as &str,
        action_type_pg as CharmActionTypePostgres,
        charm.duration() as &str,
        charm.data_source().book_title() as Option<&str>,
        charm.data_source().page_number() as Option<i16>,
        creator_id
    ).fetch_one(&mut *transaction).await.wrap_err_with(|| format!("Database error attempting to insert martial arts charm {}", charm.name())).map(|record| record.id)?;

    let charm_keywords_pg: Vec<CharmKeywordPostgres> = charm
        .keywords()
        .iter()
        .map(|keyword| (*keyword).into())
        .collect();

    query!(
        "INSERT INTO martial_arts_charms_keywords(charm_id, keyword)
        SELECT
            $1::INTEGER as charm_id,
            data.keyword as keyword
        FROM UNNEST($2::CHARMKEYWORD[]) as data(keyword)",
        charm_id as i32,
        &charm_keywords_pg as &[CharmKeywordPostgres]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error attempting to insert keywords for martial arts charm {}",
            charm.name()
        )
    })?;

    let (cost_types_pg, cost_amounts_i16) = charm.costs().iter().fold(
        (Vec::<CharmCostTypePostgres>::new(), Vec::<i16>::new()),
        |(mut cost_types_pg, mut cost_amounts_i16), (cost_type, amount)| {
            cost_types_pg.push((*cost_type).into());
            cost_amounts_i16.push((*amount).into());
            (cost_types_pg, cost_amounts_i16)
        },
    );

    query!(
        "INSERT INTO martial_arts_charms_costs(charm_id, cost_type, amount)
        SELECT
            $1::INTEGER as charm_id,
            data.cost_type as cost_type,
            data.amount as amount
        FROM UNNEST($2::CHARMCOSTTYPE[], $3::SMALLINT[]) as data(cost_type, amount)",
        charm_id as i32,
        &cost_types_pg as &[CharmCostTypePostgres],
        &cost_amounts_i16 as &[i16]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error attempting to insert activation costs for martial arts charm {}",
            charm.name()
        )
    })?;

    Ok(charm_id)
}

pub(crate) async fn create_martial_arts_charm_tree(
    transaction: &mut Transaction<'_, Postgres>,
    child_parent_pairs: &[(i32, i32)],
) -> Result<()> {
    let (child_ids, parent_ids) = child_parent_pairs.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut child_ids, mut parent_ids), (child_id, parent_id)| {
            child_ids.push(*child_id);
            parent_ids.push(*parent_id);
            (child_ids, parent_ids)
        },
    );

    query!(
        "INSERT INTO martial_arts_charm_tree(child_id, parent_id)
        SELECT
            data.child_id as child_id,
            data.parent_id as parent_id
        FROM UNNEST($1::INTEGER[], $2::INTEGER[]) as data(child_id, parent_id)",
        &child_ids as &[i32],
        &parent_ids as &[i32],
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error attemping to insert martial arts charm prerequisites trees")?;

    Ok(())
}
