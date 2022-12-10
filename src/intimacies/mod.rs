use serde::{Serialize, Deserialize};
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

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Intimacy {
    id: Option<i32>,
    pub intimacy_level: IntimacyLevel,
    pub intimacy_type: IntimacyType,
    pub description: String,
}

impl Intimacy {
    pub fn id(&self) -> Option<i32> {
        self.id
    }
}
