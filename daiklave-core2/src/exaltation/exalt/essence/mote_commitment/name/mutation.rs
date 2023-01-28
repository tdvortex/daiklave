use std::ops::Deref;

use crate::artifact::ArtifactNameMutation;

use super::{OtherMoteCommitmentName, MoteCommitmentName};

pub enum MoteCommitmentNameMutation {
    AttunedArtifact(ArtifactNameMutation),
    Other(OtherMoteCommitmentName),
}

impl From<&MoteCommitmentName<'_>> for MoteCommitmentNameMutation {
    fn from(name: &MoteCommitmentName<'_>) -> Self {
        match name {
            MoteCommitmentName::AttunedArtifact(artifact_name) => MoteCommitmentNameMutation::AttunedArtifact(artifact_name.into()),
            MoteCommitmentName::Other(other_name) => MoteCommitmentName::Other(other_name.into()),
        }
    }
}

impl Deref for MoteCommitmentNameMutation {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            MoteCommitmentName::AttunedArtifact(artifact_name_mutation) => &**artifact_name_mutation,
            MoteCommitmentName::Other(other_name) => &**other_name,
        }
    }
}

impl From<ArtifactNameMutation> for MoteCommitmentNameMutation {
    fn from(artifact_name: ArtifactNameMutation) -> Self {
        Self::AttunedArtifact(artifact_name)
    }
}

impl From<OtherMoteCommitmentName> for MoteCommitmentNameMutation {
    fn from(other: OtherMoteCommitmentName) -> Self {
        Self::Other(other)
    }
}