use crate::hearthstones::{HearthstoneId, hearthstone::{details::HearthstoneDetailsMemo, origin::HearthstoneOriginMemo}};

pub(crate) struct SlottedHearthstoneMemo {
    hearthstone_id: HearthstoneId,
    details: HearthstoneDetailsMemo,
    origin: HearthstoneOriginMemo,
}