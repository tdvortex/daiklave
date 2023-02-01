use crate::{artifact::ArtifactNameMutation, hearthstones::hearthstone::HearthstoneName, languages::language::{RemoveLanguage}};

use super::DemenseName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveMerit {
    Artifact(ArtifactNameMutation),
    Demense(DemenseName),
    ExaltedHealing,
    HearthstoneS(HearthstoneName),
    Language(RemoveLanguage),
}