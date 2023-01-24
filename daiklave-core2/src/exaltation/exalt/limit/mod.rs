mod memo;
use std::num::NonZeroU8;

pub(crate) use memo::LimitMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit<'source> {
    pub(crate) track: u8,
    pub(crate) trigger: &'source str,
}

impl<'source> Limit<'source> {
    pub(crate) fn as_memo(&self) -> LimitMemo {
        LimitMemo {
            track: self.track,
            trigger: self.trigger.to_owned(),
        }
    }

    pub fn track(&self) -> u8 {
        self.track.min(10)
    }

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
