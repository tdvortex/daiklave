use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
pub(crate) mod tables;
pub(crate) mod update;
pub use update::{compare_intimacies, IntimaciesDiff};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum IntimacyLevel {
    Minor,
    Major,
    Defining,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum IntimacyType {
    Tie,
    Principle,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Intimacies(Vec<Intimacy>);

impl Intimacies {
    pub fn new(intimacies: Vec<Intimacy>) -> Self {
        Self(intimacies)
    }
}

impl Deref for Intimacies {
    type Target = Vec<Intimacy>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Intimacies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Intimacy {
    id: Option<i32>,
    pub intimacy_level: IntimacyLevel,
    pub intimacy_type: IntimacyType,
    pub description: String,
}

impl Intimacy {
    pub fn new(
        intimacy_level: IntimacyLevel,
        intimacy_type: IntimacyType,
        description: String,
        id: Option<i32>,
    ) -> Self {
        Self {
            id,
            intimacy_level,
            intimacy_type,
            description,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }
}
