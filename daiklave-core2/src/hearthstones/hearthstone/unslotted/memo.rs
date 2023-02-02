use serde::{Deserialize, Serialize};

use crate::hearthstones::hearthstone::{
    details::HearthstoneDetailsMemo, origin::HearthstoneOriginMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct UnslottedHearthstoneMemo {
    pub details: HearthstoneDetailsMemo,
    pub origin: HearthstoneOriginMemo,
}
