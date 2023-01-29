use crate::CharacterMutation;

use super::{IntimacyTypeMemo};

/// An Intimacy to be removed from a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveIntimacy {
    pub(crate) intimacy_type: IntimacyTypeMemo,
}

impl RemoveIntimacy {
    pub fn tie(target: impl Into<String>, emotion: impl Into<String>) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Tie(target.into(), emotion.into()),
        }
    }

    pub fn principle(description: impl Into<String>) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Principle(description.into()),
        }
    }
}

impl From<RemoveIntimacy> for CharacterMutation {
    fn from(remove_intimacy: RemoveIntimacy) -> Self {
        Self::RemoveIntimacy(remove_intimacy)
    }
}
