mod memo;
pub(crate) use memo::LimitMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Limit<'source> {
    pub track: u8,
    pub trigger: &'source str,
}

impl<'source> Limit<'source> {
    pub fn as_memo(&self) -> LimitMemo {
        LimitMemo {
            track: self.track,
            trigger: self.trigger.to_owned(),
        }
    }
}
