use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// The Id for a Spirit Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpiritCharmId(pub UniqueId);
