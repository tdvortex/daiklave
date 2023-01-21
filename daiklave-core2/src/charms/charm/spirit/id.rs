use serde::{Serialize, Deserialize};

use crate::unique_id::UniqueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpiritCharmId(pub UniqueId);