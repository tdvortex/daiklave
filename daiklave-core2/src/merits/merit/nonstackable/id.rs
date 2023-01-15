use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NonStackableMeritId(pub UniqueId);