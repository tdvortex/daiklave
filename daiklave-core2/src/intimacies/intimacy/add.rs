use crate::CharacterMutation;

use super::{IntimacyLevel, IntimacyTypeMemo};

/// An Intimacy to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddIntimacy {
    pub(crate) intimacy_type: IntimacyTypeMemo,
    pub(crate) level: IntimacyLevel,
}

impl AddIntimacy {
    pub fn tie(target: impl ToString, emotion: impl ToString, level: IntimacyLevel) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Tie(target.to_string(), emotion.to_string()),
            level,
        }
    }

    pub fn principle(description: impl ToString, level: IntimacyLevel) -> Self {
        Self {
            intimacy_type: IntimacyTypeMemo::Principle(description.to_string()),
            level,
        }
    }
}

impl From<AddIntimacy> for CharacterMutation {
    fn from(add_intimacy: AddIntimacy) -> Self {
        Self::AddIntimacy(add_intimacy)
    }
}
