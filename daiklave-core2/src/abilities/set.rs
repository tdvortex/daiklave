use crate::CharacterMutation;

use super::{AbilityNameVanilla, AbilityError};

pub struct SetAbility {
    name: AbilityNameVanilla,
    dots: u8    
}

impl SetAbility {
    pub fn new(name: AbilityNameVanilla, dots: u8) -> Result<Self, AbilityError> {
        if dots > 5 {
            Err(AbilityError::InvalidRating)
        } else {
            Ok(Self {
                name,
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