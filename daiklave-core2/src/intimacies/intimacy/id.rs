use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// The Id for an Intimacy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntimacyId(pub UniqueId);
