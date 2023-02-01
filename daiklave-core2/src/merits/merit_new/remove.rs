use crate::{artifact::ArtifactNameMutation, hearthstones::hearthstone::HearthstoneName, languages::language::{RemoveLanguage}, CharacterMutation};

use super::DemenseName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveMerit {
    Artifact(ArtifactNameMutation),
    Demense(DemenseName),
    ExaltedHealing,
    HearthstoneS(HearthstoneName),
    Language(RemoveLanguage),
}

impl From<RemoveMerit> for CharacterMutation {
    fn from(remove_merit: RemoveMerit) -> Self {
        Self::RemoveMerit(remove_merit)
    }
}