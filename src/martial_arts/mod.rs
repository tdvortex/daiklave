mod tables;
use crate::{
    abilities::{Ability, AbilityName, AbilityRating, NonZeroAbility},
    charms::MartialArtsCharm,
    data_source::{BookReference, DataSource},
    id::Id,
};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub(crate) struct MartialArtsStyle {
    id: Id,
    name: String,
    description: String,
    data_source: DataSource,
}

impl PartialEq for MartialArtsStyle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl MartialArtsStyle {
    pub fn from_book(id: Id, book_title: String, page_number: i16) -> MartialArtsStyleBuilder {
        MartialArtsStyleBuilder {
            id,
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
            name: None,
            description: None,
        }
    }

    pub fn custom(id: Id, creator_id: Id) -> MartialArtsStyleBuilder {
        MartialArtsStyleBuilder {
            id,
            data_source: DataSource::Custom(creator_id),
            name: None,
            description: None,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct MartialArtsStyleBuilder {
    id: Id,
    data_source: DataSource,
    name: Option<String>,
    description: Option<String>,
}

impl MartialArtsStyleBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Result<MartialArtsStyle> {
        Ok(MartialArtsStyle {
            id: self.id,
            name: self
                .name
                .ok_or_else(|| eyre!("Martial Arts styles must be named"))?,
            description: self
                .description
                .ok_or_else(|| eyre!("Martial Arts styles must be have a description"))?,
            data_source: self.data_source,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MartialArtistDetails {
    style: MartialArtsStyle,
    rating: AbilityRating,
    charms: Vec<MartialArtsCharm>,
}

impl MartialArtistDetails {
    pub fn style(&self) -> &MartialArtsStyle {
        &self.style
    }

    pub fn as_ability(&self) -> Ability {
        Ability {
            name: AbilityName::MartialArts(self.style.name()),
            rating: &self.rating,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct MartialArtistTraits(Vec<MartialArtistDetails>);

impl MartialArtistTraits {
    pub fn get_ability(&self, style_name: &str) -> Option<Ability> {
        self.0
            .iter()
            .find(|&details| details.style().name() == style_name)
            .map(|d| d.as_ability())
    }

    pub fn add_style(&mut self, style: MartialArtsStyle, dots: u8) -> Result<()> {
        if self.0.iter().any(|details| details.style == style) {
            Err(eyre!("Style {} already known", style.name))
        } else {
            let rating = if dots == 0 {
                AbilityRating::Zero
            } else {
                AbilityRating::NonZero(NonZeroAbility {
                    dots,
                    specialties: Vec::new(),
                })
            };

            self.0.push(MartialArtistDetails {
                style,
                rating,
                charms: Default::default(),
            });
            self.0
                .sort_by(|a, b| a.style.name.as_str().cmp(b.style.name.as_str()));
            Ok(())
        }
    }

    fn get_rating_mut(&mut self, style_name: &str) -> Result<&mut AbilityRating> {
        Ok(&mut self
            .0
            .iter_mut()
            .find(|details| details.style().name() == style_name)
            .ok_or_else(|| eyre!("Martial Arts style {} not found", style_name))?
            .rating)
    }

    pub fn set_dots(&mut self, style_name: &str, dots: u8) -> Result<()> {
        self.get_rating_mut(style_name)?.set_dots(dots);
        Ok(())
    }

    pub fn add_specialty(&mut self, style_name: &str, specialty: String) -> Result<()> {
        self.get_rating_mut(style_name)?.add_specialty(specialty)
    }

    pub fn remove_specialty(&mut self, style_name: &str, specialty: &str) -> Result<()> {
        self.get_rating_mut(style_name)?.remove_specialty(specialty)
    }
}
