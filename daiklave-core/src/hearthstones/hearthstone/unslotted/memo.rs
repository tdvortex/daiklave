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

impl From<UnslottedHearthstone<'_>> for UnslottedHearthstoneMemo {
    fn from(unslotted: UnslottedHearthstone<'_>) -> Self {
        Self {
            details: (&unslotted.details).into(),
            origin: (&unslotted.origin).into(),
        }
    }
}