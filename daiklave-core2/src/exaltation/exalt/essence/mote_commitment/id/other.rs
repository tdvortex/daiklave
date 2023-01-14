use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// A unique identified for a mote commitment that is not an attuned artifact.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtherMoteCommitmentId(pub UniqueId);
