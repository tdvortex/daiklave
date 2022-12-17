use crate::{
    character::CharacterBuilder,
    charms::{
        tables::{CharmActionTypePostgres, CharmKeywordPostgres},
        CharmKeyword, MartialArtsCharm, MartialArtsCharmBuilder,
    },
    id::Id,
};
use eyre::{eyre, Context, Result};
use std::collections::HashMap;

use super::MartialArtsStyle;
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

impl CharacterBuilder {
    pub(crate) fn apply_martial_arts(
        mut self,
        style_rows: Option<Vec<MartialArtsStyleRow>>,
        character_style_rows: Option<Vec<CharacterMartialArtsRow>>,
        specialty_rows: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
        martial_arts_charm_rows: Option<Vec<MartialArtsCharmRow>>,
        charm_keyword_rows: Option<Vec<MartialArtsCharmKeywordRow>>,
    ) -> Result<Self> {
        if character_style_rows.is_none() {
            return Ok(self);
        }

        if style_rows.is_none() {
            return Err(eyre!("No styles available to apply to character"));
        }

        // Construct styles from style rows, leave space for specialties
        let mut style_map =
            style_rows
                .unwrap()
                .into_iter()
                .fold(Ok(HashMap::new()), |result_map, row| {
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

                        map.insert(style.id, (style, Vec::new()));
                        Ok(map)
                    })
                })?;

        // Construct character's specialties for styles
        if let Some(rows) = specialty_rows {
            for row in rows.into_iter() {
                if let Some(ptr) = style_map.get_mut(&Id::Database(row.style_id)) {
                    ptr.1.push(row.specialty);
                } else {
                    return Err(eyre!("Style {} not found", row.style_id));
                }
            }
        }

        // Apply styles and specialties to character
        self = character_style_rows
            .unwrap()
            .into_iter()
            .fold(Ok(self), |result_self, row| {
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
            })?;

        if martial_arts_charm_rows.is_none() {
            // No charms to build or apply
            return Ok(self);
        }

        // Construct charms except for keywords
        let mut charm_builder_map: HashMap<i32, MartialArtsCharmBuilder>;
        if let Some(rows) = martial_arts_charm_rows {
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
            return Ok(self);
        }

        // Group charm keywords
        let mut charm_keyword_map: HashMap<Id, Vec<CharmKeyword>> = HashMap::new();
        if let Some(rows) = charm_keyword_rows {
            for row in rows.into_iter() {
                let id = Id::Database(row.charm_id);
                charm_keyword_map
                    .entry(id)
                    .or_default()
                    .push(row.charm_keyword.into());
            }
        }

        for (charm_id, mut charm_builder) in charm_builder_map.into_iter() {
            if let Some(keywords) = charm_keyword_map.remove(&Id::Database(charm_id)) {
                for keyword in keywords.into_iter() {
                    charm_builder = charm_builder.with_keyword(keyword);
                }
            }
            let charm = charm_builder.build()?;

            self = self.with_martial_arts_charm(charm)?;
        }

        Ok(self)
    }
}
