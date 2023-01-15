use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// A unique Id for a Sorcery Archetype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeId(pub UniqueId);

impl Deref for SorceryArchetypeId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
