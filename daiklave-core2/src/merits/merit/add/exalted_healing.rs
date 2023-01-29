use crate::CharacterMutation;

use super::AddMerit;

pub struct AddExaltedHealing;

impl From<AddExaltedHealing> for CharacterMutation {
    fn from(add_exalted_healing: AddExaltedHealing) -> Self {
        Self::AddMerit(AddMerit::ExaltedHealing)
    }
}