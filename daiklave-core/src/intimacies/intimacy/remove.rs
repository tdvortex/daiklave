use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

use super::{IntimacyType, IntimacyTypeMemo};

/// An Intimacy to be removed from a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoveIntimacy {
    pub(crate) intimacy_type: IntimacyTypeMemo,
}

impl RemoveIntimacy {
    /// Constructs a mutation to remove the specified Intimacy.
    pub fn new(intimacy_type: IntimacyType<'_>) -> Self {
        Self {
            intimacy_type: intimacy_type.into(),
        }
    }
}

impl From<RemoveIntimacy> for CharacterMutation {
    fn from(remove_intimacy: RemoveIntimacy) -> Self {
        Self::RemoveIntimacy(remove_intimacy)
    }
}
