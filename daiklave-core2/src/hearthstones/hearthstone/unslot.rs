use crate::CharacterMutation;

use super::HearthstoneName;

pub struct UnslotHearthstone(HearthstoneName);

impl UnslotHearthstone {
    pub fn new(name: HearthstoneName) -> Self {
        Self(name)
    }
}

impl From<UnslotHearthstone> for CharacterMutation {
    fn from(unslot_hearthstone: UnslotHearthstone) -> CharacterMutation {
        CharacterMutation::UnslotHearthstone(unslot_hearthstone)
    }
}