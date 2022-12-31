use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{abilities::{Ability, AbilityView, SetAbilityError}, Character, CharacterMutationError, CharacterView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Craft(HashMap<String, Ability>);

impl Craft {
    fn set_dots(&mut self, focus: &str, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        let focus_string = focus.to_string();
        if dots == 0 {
            self.0.remove(&focus_string);
        } else {
            self.0.entry(focus_string).or_insert(Ability::Zero).set_dots(dots)?;
        }
        Ok(self)
    }
}

impl Character {
    pub fn check_set_craft_dots(&self, focus: &str, dots: u8) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else {
            Ok(())
        }
    }

    pub fn set_craft_dots(&mut self, focus: &str, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else {
            self.craft.set_dots(focus, dots);
            Ok(self)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CraftView<'source>(HashMap<&'source str, AbilityView<'source>>);

impl<'source> CraftView<'source> {
    fn set_dots(&mut self, focus: &'source str, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if dots == 0 {
            self.0.remove(focus);
        } else {
            self.0.entry(focus).or_insert(AbilityView::Zero).set_dots(dots)?;
        }
        Ok(self)
    }
}

impl<'source> CharacterView<'source> {
    pub fn check_set_craft_dots(&self, focus: &str, dots: u8) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else {
            Ok(())
        }
    }

    pub fn set_craft_dots(&mut self, focus: &'source str, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else {
            self.craft.set_dots(focus, dots);
            Ok(self)
        }
    }
}