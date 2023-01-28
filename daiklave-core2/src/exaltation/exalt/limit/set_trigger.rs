use crate::CharacterMutation;

use super::trigger::LimitTrigger;

pub struct SetLimitTrigger(LimitTrigger);

impl SetLimitTrigger {
    pub fn new(trigger: impl ToString) -> Self {
        Self(trigger.into())
    }
}

impl From<SetLimitTrigger> for CharacterMutation {
    fn from(set_limit_trigger: SetLimitTrigger) -> Self {
        Self::SetLimitTrigger(set_limit_trigger)
    }
}