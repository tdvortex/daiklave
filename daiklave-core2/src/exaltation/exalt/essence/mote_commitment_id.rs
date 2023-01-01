use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// A unique identifier for a mote commitment effect.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoteCommitmentId(pub UniqueId);

impl Deref for MoteCommitmentId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
