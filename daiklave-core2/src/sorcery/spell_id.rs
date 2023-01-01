use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::unique_id::UniqueId;

/// A unique Id for a Spell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellId(pub UniqueId);

impl Deref for SpellId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}