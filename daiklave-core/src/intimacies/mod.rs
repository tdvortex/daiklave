use std::collections::HashMap;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
pub(crate) mod diff;
pub use diff::{compare_intimacies, IntimaciesDiff};

use crate::id::IntimacyId;

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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intimacies(HashMap<IntimacyId, Intimacy>);

impl Intimacies {
    pub fn set_intimacy(&mut self, intimacy: Intimacy) {
        if let Some(old) = self.0.get_mut(&intimacy.id) {
            *old = intimacy;
        } else {
            self.0.insert(intimacy.id, intimacy);
        }
    }

    pub fn remove_intimacy(&mut self, id: IntimacyId) -> Result<()> {
        self.0.remove(&id).ok_or_else(|| eyre!("Intimacy does not exist"))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Intimacy {
    pub id: IntimacyId,
    pub intimacy_level: IntimacyLevel,
    pub intimacy_type: IntimacyType,
    pub description: String,
}

impl Intimacy {
    pub fn new(
        intimacy_level: IntimacyLevel,
        intimacy_type: IntimacyType,
        description: String,
        id: IntimacyId,
    ) -> Self {
        Self {
            id,
            intimacy_level,
            intimacy_type,
            description,
        }
    }
}
