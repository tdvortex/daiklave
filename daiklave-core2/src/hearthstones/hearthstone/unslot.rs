use crate::CharacterMutation;

use super::HearthstoneName;

/// A mutation to unslot a hearthstone from whatever artifact it is currently
/// socketed into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnslotHearthstone(pub HearthstoneName);

impl From<UnslotHearthstone> for CharacterMutation {
    fn from(unslot_hearthstone: UnslotHearthstone) -> CharacterMutation {
        CharacterMutation::UnslotHearthstone(unslot_hearthstone)
    }
}