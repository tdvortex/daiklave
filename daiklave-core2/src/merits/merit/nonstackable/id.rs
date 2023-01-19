use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// The Id for a merit which can only be purchased once per character. This is
/// the Id for both the template and the instance (since it's unique).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NonStackableMeritId(pub UniqueId);
