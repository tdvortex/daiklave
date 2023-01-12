use serde::{Deserialize, Serialize};

use crate::hearthstones::{
    hearthstone::{details::HearthstoneDetailsMemo, origin::HearthstoneOriginMemo},
    HearthstoneId,
};

use super::SlottedHearthstone;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SlottedHearthstoneMemo {
    pub hearthstone_id: HearthstoneId,
    pub details: HearthstoneDetailsMemo,
    pub origin: HearthstoneOriginMemo,
}

impl<'source> SlottedHearthstoneMemo {
    pub fn as_ref(&'source self) -> SlottedHearthstone<'source> {
        SlottedHearthstone {
            hearthstone_id: self.hearthstone_id,
            details: self.details.as_ref(),
            origin: self.origin.as_ref(),
        }
    }
}