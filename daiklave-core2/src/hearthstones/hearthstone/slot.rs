use crate::{CharacterMutation, artifact::ArtifactNameMutation};

use super::HearthstoneName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotHearthstone {
    pub artifact_name: ArtifactNameMutation,
    pub hearthstone_name: HearthstoneName,
}

impl From<SlotHearthstone> for CharacterMutation {
    fn from(slot_hearthstone: SlotHearthstone) -> Self {
        CharacterMutation::SlotHearthstone(slot_hearthstone)
    }
}