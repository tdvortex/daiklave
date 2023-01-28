use crate::CharacterMutation;

use super::CharmNameMutation;

pub struct RemoveCharm(CharmNameMutation);

impl RemoveCharm {
    pub fn new(name: CharmNameMutation) -> Self {
        Self(name)
    }
}

impl From<RemoveCharm> for CharacterMutation {
    fn from(remove_charm: RemoveCharm) -> Self {
        Self::RemoveCharm(remove_charm)
    }
}