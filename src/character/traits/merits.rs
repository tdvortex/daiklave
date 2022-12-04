use super::prerequisite::PrerequisiteSet;
use eyre::{eyre, Result};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeritType {
    Innate,
    Supernatural,
    Story,
    Purchased,
}

#[derive(Debug, Clone)]
pub struct MeritTemplate {
    id: Option<i32>,
    name: String,
    dots: u8,
    merit_type: MeritType,
    prerequisites: Vec<PrerequisiteSet>,
    description: String,
    requires_detail: bool,
}

impl MeritTemplate {
    pub fn new(name: String, dots: u8, merit_type: MeritType, prerequisites: Vec<PrerequisiteSet>, description: String, requires_detail: bool, id: Option<i32>) -> Self {
        Self {
            id,
            name,
            dots,
            merit_type,
            prerequisites,
            description,
            requires_detail,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn dots(&self) -> u8 {
        self.dots
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
}

#[derive(Debug, Clone)]
pub struct Merit {
    template: MeritTemplate,
    detail: Option<String>,
}

impl Merit {
    pub fn from_template(template: MeritTemplate, detail: Option<String>) -> Result<Self> {
        match (template.requires_detail(), detail) {
            (false, None) => Ok(Self{template, detail: None}),
            (true, None) => Err(eyre!("merit {} requires detailing", template.name())),
            (_, Some(detail_text)) => Ok(Self{template, detail: Some(detail_text)}),
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.template.id()
    }

    pub fn name(&self) -> &str {
        self.template.name()
    }

    pub fn dots(&self) -> u8 {
        self.template.dots()
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

pub type Merits = Vec<Merit>;