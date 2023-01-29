mod mutation;
pub use mutation::MoteCommitmentNameMutation;
mod other;
pub use other::OtherMoteCommitmentName;

use crate::artifact::ArtifactName;

pub enum MoteCommitmentName<'source> {
    AttunedArtifact(ArtifactName<'source>),
    Other(&'source str),
}

impl<'source> From<&'source MoteCommitmentNameMutation> for MoteCommitmentName<'source> {
    fn from(name: &'source MoteCommitmentNameMutation) -> Self {
        match name {
            MoteCommitmentNameMutation::AttunedArtifact(artifact_name) => MoteCommitmentName::AttunedArtifact(artifact_name.into()),
            MoteCommitmentNameMutation::Other(other_name) => MoteCommitmentName::Other(other_name.as_str()),
        }
    }
}