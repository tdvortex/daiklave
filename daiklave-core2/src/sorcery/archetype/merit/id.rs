use serde::{Serialize, Deserialize};

use crate::unique_id::UniqueId;

/// The Id for a merit attached to a particular Sorcery Archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeMeritId(pub UniqueId);