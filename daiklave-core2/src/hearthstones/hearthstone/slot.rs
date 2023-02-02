use crate::{
    artifact::{ArtifactName, ArtifactNameMutation},
    CharacterMutation,
};

use super::HearthstoneName;

/// A mutation to slot a hearhtstone into a specified artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotHearthstone {
    pub(crate) artifact_name: ArtifactNameMutation,
    pub(crate) hearthstone_name: HearthstoneName,
}

impl SlotHearthstone {
    /// Creates a new mutation to slot a hearthstone into an artifact.
    pub fn new(
        artifact_name: ArtifactName<'_>,
        hearthstone_name: impl Into<HearthstoneName>,
    ) -> Self {
        Self {
            artifact_name: artifact_name.into(),
            hearthstone_name: hearthstone_name.into(),
        }
    }
}

impl From<SlotHearthstone> for CharacterMutation {
    fn from(slot_hearthstone: SlotHearthstone) -> Self {
        CharacterMutation::SlotHearthstone(slot_hearthstone)
    }
}
