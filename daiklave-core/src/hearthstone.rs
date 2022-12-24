use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use eyre::{eyre, Result};

use crate::{id::{HearthstoneId, CharacterId, OwnedHearthstoneId}, data_source::{DataSource, BookReference}};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HearthstoneCategory {
    Air,
    Earth,
    Fire,
    Water,
    Wood,
    Solar,
    Sidereal,
    Lunar,
    Abyssal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HearthstoneKeyword {
    Linked,
    Steady,
    Dependent,
    ManseBorn,
    WildBorn,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GeomancyLevel {
    Standard,
    Greater,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum HearthstoneSource {
    None,
    Desmense(String),
    ManseAndDemesne(String, String), // Demense, Manse
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hearthstone {
    id: HearthstoneId,
    name: String,
    lore: Option<String>,
    powers: Option<String>,
    data_source: DataSource,
    category: HearthstoneCategory,
    keywords: HashSet<HearthstoneKeyword>,
    geomancy_level: GeomancyLevel,
}

impl PartialEq for Hearthstone {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Hearthstone {}

impl Hearthstone {
    pub fn from_book(id: HearthstoneId, book_title: String, page_number: i16) -> HearthstoneBuilder {
        HearthstoneBuilder {
            id,
            name: None,
            lore: None,
            powers: None,
            data_source: DataSource::Book(BookReference{book_title, page_number}),
            category: None,
            keywords: HashSet::new(),
            geomancy_level: None,
        }
    }

    pub fn custom(id: HearthstoneId, creator_id: CharacterId) -> HearthstoneBuilder {
        HearthstoneBuilder {
            id,
            name: None,
            lore: None,
            powers: None,
            data_source: DataSource::Custom(creator_id),
            category: None,
            keywords: HashSet::new(),
            geomancy_level: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OwnedHearthstone {
    id: OwnedHearthstoneId,
    hearthstone: Hearthstone,
    source: HearthstoneSource,
}

impl OwnedHearthstone {
    pub fn id(&self) -> OwnedHearthstoneId {
        self.id
    }
}

impl OwnedHearthstone {
    pub fn new(id: OwnedHearthstoneId, hearthstone: Hearthstone, source: HearthstoneSource) -> Self {
        Self {
            id,
            hearthstone,
            source
        }
    }
}

pub struct HearthstoneBuilder {
    id: HearthstoneId,
    name: Option<String>,
    lore: Option<String>,
    powers: Option<String>,
    data_source: DataSource,
    category: Option<HearthstoneCategory>,
    keywords: HashSet<HearthstoneKeyword>,
    geomancy_level: Option<GeomancyLevel>,
}

impl HearthstoneBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    pub fn with_powers(mut self, powers: String) -> Self {
        self.powers = Some(powers);
        self
    }

    pub fn with_category(mut self, category: HearthstoneCategory) -> Self {
        self.category = Some(category);
        self
    }

    pub fn with_keyword(mut self, keyword: HearthstoneKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    pub fn with_geomancy_level(mut self, geomancy_level: GeomancyLevel) -> Self {
        self.geomancy_level = Some(geomancy_level);
        self
    }

    pub fn build(self) -> Result<Hearthstone> {
        Ok(Hearthstone { 
            id: self.id, 
            name: self.name.ok_or_else(|| eyre!("Hearthstones must have a name"))?, 
            lore: self.lore, 
            powers: self.powers, 
            data_source: self.data_source, 
            category: self.category.ok_or_else(|| eyre!("Hearthstones must have a category"))?, 
            keywords: self.keywords, 
            geomancy_level: self.geomancy_level.ok_or_else(|| eyre!("Hearthstones must be specified as standard or greater"))? })
    }
}