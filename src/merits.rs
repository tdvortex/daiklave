use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeritType {
    Innate,
    Supernatural,
    Story,
    Purchased,
}

#[derive(Debug, Eq)]
pub struct SimpleMerit {
    name: String,
    dots: u8,
    merit_type: MeritType,
    description: String,
}

impl PartialEq for SimpleMerit {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.dots == other.dots
    }
}

impl Hash for SimpleMerit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.dots.hash(state);
    }
}

impl std::fmt::Display for SimpleMerit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dots = String::from_utf16(&vec![0x2022; self.dots as usize]).unwrap();
        write!(f, "{} ({})", self.name, dots)
    }
}

impl SimpleMerit {
    fn dots(&self) -> u8 {
        self.dots
    }

    fn merit_type(&self) -> &MeritType {
        &self.merit_type
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }
}

#[derive(Debug, Eq)]
pub struct DetailedMerit {
    name: String,
    detail: String,
    dots: u8,
    merit_type: MeritType,
    description: String,
}

impl PartialEq for DetailedMerit {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.dots == other.dots && self.detail == other.detail
    }
}

impl Hash for DetailedMerit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.detail.hash(state);
        self.dots.hash(state);
    }
}

impl std::fmt::Display for DetailedMerit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dots = String::from_utf16(&vec![0x2022; self.dots as usize]).unwrap();
        write!(f, "{} ({}) ({})", self.name, self.detail, dots)
    }
}

impl DetailedMerit {
    fn dots(&self) -> u8 {
        self.dots
    }

    fn merit_type(&self) -> &MeritType {
        &self.merit_type
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Merit {
    Simple(SimpleMerit),
    Detailed(DetailedMerit),
}

impl std::fmt::Display for Merit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(simple) => simple.fmt(f),
            Self::Detailed(detailed) => detailed.fmt(f),
        }
    }
}

impl Merit {
    pub fn dots(&self) -> u8 {
        match self {
            Self::Simple(simple) => simple.dots(),
            Self::Detailed(detailed) => detailed.dots(),
        }
    }

    pub fn merit_type(&self) -> &MeritType {
        match self {
            Self::Simple(simple) => simple.merit_type(),
            Self::Detailed(detailed) => detailed.merit_type(),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Simple(simple) => simple.description(),
            Self::Detailed(detailed) => detailed.description(),
        }
    }
}

impl Merit {
    pub fn new(
        name: String,
        dots: u8,
        merit_type: MeritType,
        description: String,
        maybe_detail: Option<String>,
    ) -> Self {
        if let Some(detail) = maybe_detail {
            Self::Detailed(DetailedMerit {
                name,
                detail,
                dots,
                merit_type,
                description,
            })
        } else {
            Self::Simple(SimpleMerit {
                name,
                dots,
                merit_type,
                description,
            })
        }
    }
}

pub type Merits = HashSet<Merit>;
