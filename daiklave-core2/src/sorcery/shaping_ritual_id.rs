use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::unique_id::UniqueId;

/// A unique Id for a ShapingRitual
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShapingRitualId(pub UniqueId);

impl Deref for ShapingRitualId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}