use serde::{Serialize, Deserialize};

use super::Limit;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LimitMemo {
    pub track: u8,
    pub trigger: String,
}

impl<'source> LimitMemo {
    pub fn as_ref(&'source self) -> Limit<'source> {
        Limit { track: self.track, trigger: self.trigger.as_str() }
    }
}