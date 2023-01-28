mod mutation;
use std::ops::Deref;

pub use mutation::MoteCommitmentNameMutation;
mod other;
pub use other::OtherMoteCommitmentName;

use crate::artifact::ArtifactName;

pub enum MoteCommitmentName<'source> {
    AttunedArtifact(ArtifactName<'source>),
    Other(&'source str),
}

impl<'any, 'source> Deref for MoteCommitmentName<'source> {
    type Target = str;

    fn deref(&'any self) -> &'source Self::Target {
        match self {
            MoteCommitmentName::AttunedArtifact(artifact_name) => &**artifact_name,
            MoteCommitmentName::Other(other_name) => *other_name,
        }
    }
}

impl<'source> From<&'source MoteCommitmentNameMutation> for MoteCommitmentName<'source> {
    fn from(name: &'source MoteCommitmentNameMutation) -> Self {
        match name {
            MoteCommitmentNameMutation::AttunedArtifact(artifact_name) => MoteCommitmentName::AttunedArtifact(artifact_name.as_str()),
            MoteCommitmentNameMutation::Other(other_name) => MoteCommitmentName::Other(other_name.as_str()),
        }
    }
}