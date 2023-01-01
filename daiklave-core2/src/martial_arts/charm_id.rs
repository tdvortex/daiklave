use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// A unique identifier for a Martial Arts Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MartialArtsCharmId(pub UniqueId);

impl Deref for MartialArtsCharmId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
