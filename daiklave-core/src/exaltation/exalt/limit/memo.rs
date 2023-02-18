use serde::{Deserialize, Serialize};

use super::{Limit, LimitTrigger};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LimitMemo {
    pub track: u8,
    pub trigger: LimitTrigger,
}

#[allow(clippy::from_over_into)]
impl<'source> Into<Limit<'source>> for &'source LimitMemo {
    fn into(self) -> Limit<'source> {
        Limit {
            track: self.track,
            trigger: &self.trigger,
        }
    }
}

impl From<&Limit<'_>> for LimitMemo {
    fn from(value: &Limit<'_>) -> Self {
        Self {
            track: value.track,
            trigger: value.trigger.into(),
        }
    }
}