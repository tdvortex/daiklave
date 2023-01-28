use crate::{CharacterMutation, artifact::ArtifactNameMutation};

use super::HearthstoneName;

pub struct SlotHearthstone {
    artifact_name: ArtifactNameMutation,
    hearthstone_name: HearthstoneName,
}

impl SlotHearthstone {
    pub fn new(artifact_name: ArtifactNameMutation, hearthstone_name: HearthstoneName) -> Self {
        Self {
            artifact_name,
            hearthstone_name,
        }
    }
}

impl From<SlotHearthstone> for CharacterMutation {
    fn from(slot_hearthstone: SlotHearthstone) -> Self {
        CharacterMutation::SlotHearthstone(slot_hearthstone)
    }
}