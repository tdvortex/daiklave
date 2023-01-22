mod craft_memo;

use std::collections::HashMap;

pub(crate) use craft_memo::CraftMemo;

use crate::{abilities::AbilityRating, CharacterMutationError};
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Craft<'source>(pub(crate) HashMap<&'source str, AbilityRating<'source>>);

impl<'source> Craft<'source> {
    pub(crate) fn as_memo(&self) -> CraftMemo {
        CraftMemo(
            self.0
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_memo()))
                .collect(),
        )
    }

    pub fn set_dots(
        &mut self,
        focus: &'source str,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots == 0 {
            self.0.remove(focus);
        } else {
            self.0
                .entry(focus)
                .or_insert(AbilityRating::Zero)
                .set_dots(dots)?;
        }
        Ok(self)
    }

    pub fn dots(&self, focus: &str) -> u8 {
        self.0.get(focus).map_or(0, |ability| ability.dots())
    }

    pub fn max(&self) -> u8 {
        self.0.values().map(|rating| rating.dots()).max().unwrap_or(0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        let mut vec: Vec<&str> = self.0.keys().copied().collect();
        vec.sort();
        vec.into_iter()
    }
}
