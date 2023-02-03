use serde::{Deserialize, Serialize};

use crate::artifact::{ArtifactName, AttuneArtifact};

/// Indicates whether motes are spent/committed from peripheral or peripheral
/// pool first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MotePoolName {
    /// Spend/commit peripheral motes first
    Peripheral,
    /// Spend/commit personal motes first
    Personal,
}

impl<'source> MotePoolName {
    /// Attune to an artifact using this mote pool first.
    pub fn attune_using(self, artifact: ArtifactName<'source>) -> AttuneArtifact {
        AttuneArtifact {
            artifact_name: artifact.into(),
            first: self,
        }
    }
}
