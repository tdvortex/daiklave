use serde::{Serialize, Deserialize};

use crate::unique_id::UniqueId;

/// The Id of a Solar Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SolarCharmId(pub UniqueId);