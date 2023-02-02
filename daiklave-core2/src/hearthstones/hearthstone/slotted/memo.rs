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

impl From<&SlottedHearthstone<'_>> for SlottedHearthstoneMemo {
    fn from(view: &SlottedHearthstone<'_>) -> Self {
        Self {
            name: view.name.to_owned(),
            details: (&view.details).into(),
            origin: (&view.origin).into(),
        }
    }
}
