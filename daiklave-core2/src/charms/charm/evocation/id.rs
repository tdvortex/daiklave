use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// The Id of an Evocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvocationId(pub UniqueId);
