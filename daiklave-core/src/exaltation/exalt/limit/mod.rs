mod gain;
mod reduce;
mod set_trigger;
mod trigger;
pub use gain::GainLimit;
pub use reduce::ReduceLimit;
pub use set_trigger::SetLimitTrigger;
pub use trigger::LimitTrigger;

mod memo;
use std::num::NonZeroU8;

pub(crate) use memo::LimitMemo;

/// The Limit track and trigger of a Celestial Exalted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit<'source> {
    pub(crate) track: u8,
    pub(crate) trigger: &'source str,
}

impl<'source> Limit<'source> {
    /// The current amount of Limit the character possesses.
    pub fn track(&self) -> u8 {
        self.track.min(10)
    }

    /// The character's Limit Trigger.
    pub fn trigger(&self) -> &'source str {
        self.trigger
    }

    pub(crate) fn gain_limit(&mut self, amount: NonZeroU8) {
        let max_add = self.track - 10.min(self.track);

        self.track += amount.get().min(max_add);
    }

    pub(crate) fn remove_limit(&mut self, amount: NonZeroU8) {
        self.track -= amount.get().min(self.track);
    }

    pub(crate) fn set_trigger(&mut self, trigger: &'source str) {
        self.trigger = trigger;
    }
}
