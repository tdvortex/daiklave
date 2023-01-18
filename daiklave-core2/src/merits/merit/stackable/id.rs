use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// The Id for a specific instance of a stackable merit. For example, this
/// might be the Id for "Mentor (Sha'a Okaa)".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StackableMeritId(pub UniqueId);

/// The Id for a specific stackable merit. For example, this might be the
/// Id for "Allies".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StackableMeritTemplateId(pub UniqueId);
