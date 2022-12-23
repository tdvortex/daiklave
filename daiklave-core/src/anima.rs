use serde::{Deserialize, Serialize};

use crate::{
    data_source::{BookReference, DataSource},
    id::{AnimaEffectId, CharacterId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AnimaLevel {
    Dim,
    Glowing,
    Burning,
    Bonfire,
}

impl AnimaLevel {
    pub fn increase(&mut self) {
        let new_rating = match self {
            AnimaLevel::Dim => AnimaLevel::Glowing,
            AnimaLevel::Glowing => AnimaLevel::Burning,
            AnimaLevel::Burning => AnimaLevel::Bonfire,
            AnimaLevel::Bonfire => AnimaLevel::Bonfire,
        };
        *self = new_rating;
    }

    pub fn decrease(&mut self) {
        let new_rating = match self {
            AnimaLevel::Dim => AnimaLevel::Dim,
            AnimaLevel::Glowing => AnimaLevel::Dim,
            AnimaLevel::Burning => AnimaLevel::Glowing,
            AnimaLevel::Bonfire => AnimaLevel::Burning,
        };
        *self = new_rating;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExaltAnimaType {
    AnySolar,
    DawnSolar,
    ZenithSolar,
    TwilightSolar,
    NightSolar,
    EclipseSolar,
    Custom,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct AnimaEffect {
    id: AnimaEffectId,
    data_source: DataSource,
    exalt_type: ExaltAnimaType,
    description: String,
}

impl PartialEq for AnimaEffect {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl AnimaEffect {
    pub fn from_book(
        id: AnimaEffectId,
        book_title: String,
        page_number: i16,
        exalt_and_caste: ExaltAnimaType,
        description: String,
    ) -> Self {
        Self {
            id,
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
            exalt_type: exalt_and_caste,
            description,
        }
    }

    pub fn custom(
        id: AnimaEffectId,
        creator_id: CharacterId,
        exalt_and_caste: ExaltAnimaType,
        description: String,
    ) -> Self {
        Self {
            id,
            data_source: DataSource::Custom(creator_id),
            exalt_type: exalt_and_caste,
            description,
        }
    }

    pub fn id(&self) -> AnimaEffectId {
        self.id
    }

    pub fn data_source(&self) -> &DataSource {
        &self.data_source
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn exalt_and_caste(&self) -> ExaltAnimaType {
        self.exalt_type
    }
}
