use crate::CharacterMutation;

use super::CharmNameMutation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveCharm(pub(crate) CharmNameMutation);

impl From<RemoveCharm> for CharacterMutation {
    fn from(remove_charm: RemoveCharm) -> Self {
        Self::RemoveCharm(remove_charm)
    }
}