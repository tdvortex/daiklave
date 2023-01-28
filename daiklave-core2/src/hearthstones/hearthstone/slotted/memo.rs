use serde::{Deserialize, Serialize};

use crate::hearthstones::hearthstone::{
    details::HearthstoneDetailsMemo, origin::HearthstoneOriginMemo,
};

use super::SlottedHearthstone;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SlottedHearthstoneMemo {
    pub name: String,
    pub details: HearthstoneDetailsMemo,
    pub origin: HearthstoneOriginMemo,
}

impl<'source> SlottedHearthstoneMemo {
    pub fn as_ref(&'source self) -> SlottedHearthstone<'source> {
        SlottedHearthstone {
            name: self.name.as_str(),
            details: self.details.as_ref(),
            origin: self.origin.as_ref(),
        }
    }
}
