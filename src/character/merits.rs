use std::collections::HashSet;

#[derive(Debug)]
enum MeritType {
    Innate,
    Supernatural,
    Story,
    Purchased,
}

trait MeritProperties: std::fmt::Display {
    fn dots(&self) -> u8;
    fn merit_type(&self) -> &MeritType;
    fn description(&self) -> &str;
}

#[derive(Debug)]
pub struct SimpleMerit {
    name: String,
    dots: u8,
    merit_type: MeritType,
    description: String,
}

impl std::fmt::Display for SimpleMerit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dots = String::from_utf16(&vec![0x2022; self.dots as usize]).unwrap();
        write!(f, "{}, ({})", self.name, dots)
    }
}

impl MeritProperties for SimpleMerit {
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

#[derive(Debug)]
pub struct DetailedMerit {
    name: String,
    detail: String,
    dots: u8,
    merit_type: MeritType,
    description: String,
}

impl std::fmt::Display for DetailedMerit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dots = String::from_utf16(&vec![0x2022; self.dots as usize]).unwrap();
        write!(f, "{}, ({}), ({})", self.name, self.detail, dots)
    }
}

impl MeritProperties for DetailedMerit {
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

#[derive(Debug)]
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

impl MeritProperties for Merit {
    fn dots(&self) -> u8 {
        match self {
            Self::Simple(simple) => simple.dots(),
            Self::Detailed(detailed) => detailed.dots(),
        }
    }

    fn merit_type(&self) -> &MeritType {
        match self {
            Self::Simple(simple) => simple.merit_type(),
            Self::Detailed(detailed) => detailed.merit_type(),
        }
    }

    fn description(&self) -> &str {
        match self {
            Self::Simple(simple) => simple.description(),
            Self::Detailed(detailed) => detailed.description(),
        }
    }
}

impl Merit {
    fn new(
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
