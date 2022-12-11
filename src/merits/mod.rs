pub(crate) mod update;
use serde::{Deserialize, Serialize};
pub use update::MeritDiff;
pub(crate) mod create;
pub(crate) mod tables;
use crate::{
    data_source::{BookReference, DataSource},
    prerequisite::PrerequisiteSet,
};
use eyre::{eyre, Result};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum MeritType {
    Innate,
    Supernatural,
    Story,
    Purchased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeritTemplate {
    id: Option<i32>,
    name: String,
    merit_type: MeritType,
    prerequisites: Vec<PrerequisiteSet>,
    description: String,
    requires_detail: bool,
    data_source: DataSource,
}

impl MeritTemplate {
    pub fn create_from_book(book_title: String, page_number: i16) -> MeritTemplateBuilder {
        MeritTemplateBuilder {
            name: Default::default(),
            merit_type: Default::default(),
            prerequisites: Default::default(),
            description: Default::default(),
            requires_detail: Default::default(),
            id: Default::default(),
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
        }
    }

    pub fn create_custom(creator_id: Option<i32>) -> MeritTemplateBuilder {
        MeritTemplateBuilder {
            name: Default::default(),
            merit_type: Default::default(),
            prerequisites: Default::default(),
            description: Default::default(),
            requires_detail: Default::default(),
            id: Default::default(),
            data_source: DataSource::Custom(creator_id),
        }
    }

    pub fn id(&self) -> Option<i32> {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merit {
    id: Option<i32>,
    template: MeritTemplate,
    dots: u8,
    detail: Option<String>,
}

impl Merit {
    pub(crate) fn from_template(
        template: MeritTemplate,
        dots: u8,
        detail: Option<String>,
        id: Option<i32>,
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

    pub fn instance_id(&self) -> Option<i32> {
        self.id
    }

    pub fn template_id(&self) -> Option<i32> {
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
    id: Option<i32>,
    data_source: DataSource,
}

impl MeritTemplateBuilder {
    pub(crate) fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
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
