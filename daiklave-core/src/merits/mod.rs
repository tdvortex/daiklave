pub(crate) mod diff;
use std::ops::{Deref, DerefMut};

use crate::{
    data_source::{BookReference, DataSource},
    id::{CharacterId, Id},
    prerequisite::PrerequisiteSet,
};
pub use diff::{compare_merits, MeritDiff};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum MeritType {
    Innate,
    Supernatural,
    Story,
    Purchased,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct MeritTemplate {
    id: Id,
    name: String,
    merit_type: MeritType,
    prerequisites: Vec<PrerequisiteSet>,
    description: String,
    requires_detail: bool,
    data_source: DataSource,
}

impl PartialEq for MeritTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl MeritTemplate {
    pub fn from_book(id: Id, book_title: String, page_number: i16) -> MeritTemplateBuilder {
        MeritTemplateBuilder {
            name: Default::default(),
            merit_type: Default::default(),
            prerequisites: Default::default(),
            description: Default::default(),
            requires_detail: Default::default(),
            id,
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
        }
    }

    pub fn custom(id: Id, creator_id: CharacterId) -> MeritTemplateBuilder {
        MeritTemplateBuilder {
            name: Default::default(),
            merit_type: Default::default(),
            prerequisites: Default::default(),
            description: Default::default(),
            requires_detail: Default::default(),
            id,
            data_source: DataSource::Custom(creator_id),
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn merit_type(&self) -> MeritType {
        self.merit_type
    }

    pub fn prerequisites(&self) -> &Vec<PrerequisiteSet> {
        &self.prerequisites
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn requires_detail(&self) -> bool {
        self.requires_detail
    }

    pub fn data_source(&self) -> &DataSource {
        &self.data_source
    }
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Merit {
    id: Id,
    template: MeritTemplate,
    dots: u8,
    detail: Option<String>,
}

impl PartialEq for Merit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Merit {
    pub fn from_template(
        template: MeritTemplate,
        dots: u8,
        detail: Option<String>,
        id: Id,
    ) -> Result<Self> {
        match (template.requires_detail(), detail) {
            (false, None) => Ok(Self {
                id,
                template,
                dots,
                detail: None,
            }),
            (true, None) => Err(eyre!("Missing detail for merit {}", template.name())),
            (_, Some(detail_text)) => Ok(Self {
                id,
                template,
                dots,
                detail: Some(detail_text),
            }),
        }
    }

    pub fn instance_id(&self) -> Id {
        self.id
    }

    pub fn template_id(&self) -> Id {
        self.template.id()
    }

    pub fn name(&self) -> &str {
        self.template.name()
    }

    pub fn dots(&self) -> u8 {
        self.dots
    }

    pub fn merit_type(&self) -> MeritType {
        self.template.merit_type()
    }

    pub fn prerequisites(&self) -> &Vec<PrerequisiteSet> {
        self.template.prerequisites()
    }

    pub fn description(&self) -> &str {
        self.template.description()
    }

    pub fn requires_detail(&self) -> bool {
        self.template.requires_detail()
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    pub fn data_source(&self) -> &DataSource {
        self.template.data_source()
    }
}

impl std::fmt::Display for Merit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dots = String::from_utf16(&vec![0x2022; self.dots() as usize]).unwrap();
        if let Some(detail_text) = self.detail() {
            write!(f, "{} ({}) ({})", self.name(), detail_text, dots)
        } else {
            write!(f, "{} ({})", self.name(), dots)
        }
    }
}

#[derive(Debug)]
pub struct MeritTemplateBuilder {
    name: Option<String>,
    merit_type: Option<MeritType>,
    prerequisites: Vec<PrerequisiteSet>,
    description: Option<String>,
    requires_detail: Option<bool>,
    id: Id,
    data_source: DataSource,
}

impl MeritTemplateBuilder {
    pub fn with_database_id(mut self, id: i32) -> Self {
        self.id = Id::Database(id);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_merit_type(mut self, merit_type: MeritType) -> Self {
        self.merit_type = Some(merit_type);
        self
    }

    pub fn requiring_detail(mut self) -> Self {
        self.requires_detail = Some(true);
        self
    }

    pub fn not_requiring_detail(mut self) -> Self {
        self.requires_detail = Some(false);
        self
    }

    pub fn with_prerequisite_set(mut self, prerequisite_set: PrerequisiteSet) -> Self {
        self.prerequisites.push(prerequisite_set);
        self
    }

    pub fn build(self) -> Result<MeritTemplate> {
        Ok(MeritTemplate {
            id: self.id,
            name: self.name.ok_or_else(|| eyre!("merit name is required"))?,
            merit_type: self.merit_type.ok_or_else(|| {
                eyre!("merit must be one of Innate, Purchased, Story, or Supernatural")
            })?,
            prerequisites: self.prerequisites,
            description: self
                .description
                .ok_or_else(|| eyre!("Merit must have a description"))?,
            requires_detail: self
                .requires_detail
                .ok_or_else(|| eyre!("Merit must specify if detail is required"))?,
            data_source: self.data_source,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Merits(Vec<Merit>);

impl Merits {
    pub fn new(merits: Vec<Merit>) -> Self {
        Self(merits)
    }
}

impl Deref for Merits {
    type Target = Vec<Merit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Merits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Flaw;
