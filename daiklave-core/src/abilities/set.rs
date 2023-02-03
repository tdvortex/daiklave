use crate::CharacterMutation;

use super::{AbilityError, AbilityNameQualified, AbilityNameQualifiedMutation};

/// A mutation to set a specific ability to a dot level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAbility {
    pub(crate) name: AbilityNameQualifiedMutation,
    pub(crate) dots: u8,
}

impl SetAbility {
    /// Creates a new SetAbility mutation.
    pub fn new(name: AbilityNameQualified<'_>, dots: u8) -> Result<Self, AbilityError> {
        if dots > 5 {
            Err(AbilityError::InvalidRating)
        } else {
            Ok(Self {
                name: name.into(),
                dots,
            })
        }
    }
}

impl From<SetAbility> for CharacterMutation {
    fn from(set_ability: SetAbility) -> Self {
        CharacterMutation::SetAbility(set_ability)
    }
}
