use crate::CharacterMutation;

use super::trigger::LimitTrigger;

/// A mutation to set the Limit Trigger for a Celestial Exalt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetLimitTrigger(pub LimitTrigger);

impl From<SetLimitTrigger> for CharacterMutation {
    fn from(set_limit_trigger: SetLimitTrigger) -> Self {
        Self::SetLimitTrigger(set_limit_trigger)
    }
}