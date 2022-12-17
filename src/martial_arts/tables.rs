use crate::{
    character::CharacterBuilder,
    charms::{
        tables::{CharmActionTypePostgres, CharmKeywordPostgres},
        MartialArtsCharm, MartialArtsCharmBuilder,
    },
    id::Id,
};
use eyre::{eyre, Context, Result};
use std::collections::HashMap;

use super::MartialArtsStyle;

pub(crate) struct MartialArtsStyleRow {
    id: i32,
    name: String,
    description: String,
    book_name: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

pub(crate) struct CharacterMartialArtsRow {
    character_id: i32,
    style_id: i32,
    dots: i16,
}

pub(crate) struct CharacterMartialArtsSpecialtyRow {
    character_id: i32,
    style_id: i32,
    specialty: String,
}

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
    book_name: Option<String>,
    page_number: Option<i16>,
    creator_id: Option<i32>,
}

pub(crate) struct MartialArtsCharmKeywordRow {
    charm_id: i32,
    charm_keyword: CharmKeywordPostgres,
}

pub(crate) struct MartialArtsStyleWeaponRow {
    style_id: i32,
    weapon_id: i32,
}

impl CharacterBuilder {
    pub(crate) fn apply_martial_arts(
        mut self,
        style_rows: Option<Vec<MartialArtsStyleRow>>,
        character_style_rows: Option<Vec<CharacterMartialArtsRow>>,
        specialty_rows: Option<Vec<CharacterMartialArtsSpecialtyRow>>,
        charm_rows: Option<Vec<MartialArtsCharmRow>>,
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
                        let builder = if row.book_name.is_some()
                            && row.page_number.is_some()
                            && row.creator_id.is_none()
                        {
                            MartialArtsStyle::from_book(
                                Id::Database(row.id),
                                row.book_name.unwrap(),
                                row.page_number.unwrap(),
                            )
                        } else if row.book_name.is_none()
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

        if charm_rows.is_none() {
            // No charms to build or apply
            return Ok(self);
        }

        // Construct charms except for keywords
        let mut charm_builder_map: HashMap<i32, (i32, MartialArtsCharmBuilder)>;
        if let Some(rows) = charm_rows {
            charm_builder_map = HashMap::new();
            for row in rows.into_iter() {
                let mut builder = if row.book_name.is_some()
                    && row.page_number.is_some()
                    && row.creator_id.is_none()
                {
                    MartialArtsCharm::from_book(
                        Id::Database(row.id),
                        row.book_name.unwrap(),
                        row.page_number.unwrap(),
                    )
                } else if row.book_name.is_none()
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
                    .with_name(row.name)
                    .with_action_type(row.action_type.into())
                    .with_description(row.description)
                    .with_duration(row.duration)
                    .requiring_martial_arts_dots(martial_arts_dots)
                    .requiring_essence(essence_rating);

                if let Some(summary) = row.summary {
                    builder = builder.with_summary(summary);
                }

                let style_id = row.style_id;

                charm_builder_map.insert(row.id, (style_id, builder));
            }
        } else {
            return Ok(self);
        }

        // Group charm keywords
        let mut charm_keyword_map = HashMap::new();
        if let Some(rows) = charm_keyword_rows {
            for row in rows.into_iter() {
                let id = Id::Database(row.charm_id);
                charm_keyword_map
                    .entry(id)
                    .or_insert(Vec::new())
                    .push(row.charm_keyword.into());
            }
        }

        for (charm_id, (style_id, mut charm_builder)) in charm_builder_map.into_iter() {
            if let Some(keywords) = charm_keyword_map.remove(&Id::Database(charm_id)) {
                for keyword in keywords.into_iter() {
                    charm_builder = charm_builder.with_keyword(keyword);
                }
            }
            let charm = charm_builder.build()?;

            self = self.with_martial_arts_charm(Id::Database(style_id), charm)?;
        }

        Ok(self)
    }
}
