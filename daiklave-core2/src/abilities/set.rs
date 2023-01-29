use crate::{CharacterMutation, craft::CraftName, martial_arts::style::MartialArtsStyleName};

use super::{AbilityError, ability::AbilityNameVanilla, AbilityNameQualifiedMutation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAbility {
    pub(crate) name: AbilityNameQualifiedMutation,
    pub(crate) dots: u8    
}

impl SetAbility {
    pub fn vanilla(name: AbilityNameVanilla, dots: u8) -> Result<Self, AbilityError> {
        if dots > 5 {
            Err(AbilityError::InvalidRating)
        } else {
            Ok(Self { name: name.into(), dots })
        }
    }

    pub fn craft(focus: CraftName, dots: u8) -> Result<Self, AbilityError> {
        if dots > 5 {
            Err(AbilityError::InvalidRating)
        } else {
            Ok(Self { name: focus.into(), dots })
        }
    }

    pub fn martial_arts(style: MartialArtsStyleName, dots: u8) -> Result<Self, AbilityError> {
        if dots > 5 {
            Err(AbilityError::InvalidRating)
        } else {
            Ok(Self { name: style.into(), dots })
        }
    }
}

impl From<SetAbility> for CharacterMutation {
    fn from(set_ability: SetAbility) -> Self {
        CharacterMutation::SetAbility(set_ability)
    }
}