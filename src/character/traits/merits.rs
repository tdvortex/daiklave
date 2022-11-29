use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeritType {
    Innate,
    Supernatural,
    Story,
    Purchased,
}

#[derive(Debug, Eq, Clone)]
pub struct Merit {
    pub name: String,
    pub detail: Option<String>,
    pub dots: u8,
    pub merit_type: MeritType,
    pub description: String,
}

impl Hash for Merit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.detail.hash(state);
        self.dots.hash(state);
    }
}

impl std::fmt::Display for Merit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dots = String::from_utf16(&vec![0x2022; self.dots as usize]).unwrap();
        if let Some(detail_text) = &self.detail {
            write!(f, "{} ({}) ({})", self.name, detail_text, dots)
        } else {
            write!(f, "{} ({})", self.name, dots)
        }
    }
}

impl PartialEq for Merit {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.dots == other.dots && self.detail == other.detail
    }
}

impl Merit {
    pub fn new(
        name: String,
        dots: u8,
        merit_type: MeritType,
        description: String,
        detail: Option<String>,
    ) -> Self {
        Self {
            name,
            detail,
            dots,
            merit_type,
            description,
        }
    }
}

pub type Merits = HashSet<Merit>;
