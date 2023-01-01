use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{abilities::Ability, CharacterMutationError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Craft(HashMap<String, Ability>);

impl Craft {
    pub fn set_dots(&mut self, focus: &str, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        let focus_string = focus.to_string();
        if dots == 0 {
            self.0.remove(&focus_string);
        } else {
            self.0
                .entry(focus_string)
                .or_insert(Ability::Zero)
                .set_dots(dots)?;
        }
        Ok(self)
    }

    pub fn dots(&self, focus: &str) -> u8 {
        self.0.get(focus).map_or(0, |ability| ability.dots())
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|s| s.as_str())
    }
}
