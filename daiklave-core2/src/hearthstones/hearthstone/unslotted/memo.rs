use serde::{Deserialize, Serialize};

use crate::hearthstones::hearthstone::{
    details::HearthstoneDetailsMemo, origin::HearthstoneOriginMemo,
};

use super::UnslottedHearthstone;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct UnslottedHearthstoneMemo {
    pub details: HearthstoneDetailsMemo,
    pub origin: HearthstoneOriginMemo,
}

impl<'source> UnslottedHearthstoneMemo {
    pub fn as_ref(&'source self) -> UnslottedHearthstone<'source> {
        UnslottedHearthstone {
            details: self.details.as_ref(),
            origin: self.origin.as_ref(),
        }
    }
}
