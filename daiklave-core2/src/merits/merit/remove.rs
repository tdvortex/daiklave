use crate::{hearthstones::hearthstone::HearthstoneName, languages::language::{RemoveLanguage}, CharacterMutation, artifact::RemoveArtifact};

use super::DemenseName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveMerit {
    Artifact(RemoveArtifact),
    Demense(DemenseName),
    ExaltedHealing,
    HearthstoneS(HearthstoneName),
    Language(RemoveLanguage),
}

impl From<RemoveArtifact> for RemoveMerit {
    fn from(remove_artifact: RemoveArtifact) -> Self {
        Self::Artifact(remove_artifact)
    }
}

impl From<RemoveLanguage> for RemoveMerit {
    fn from(remove_language: RemoveLanguage) -> Self {
        Self::Language(remove_language)
    }
}

impl From<RemoveMerit> for CharacterMutation {
    fn from(remove_merit: RemoveMerit) -> Self {
        Self::RemoveMerit(remove_merit)
    }
}