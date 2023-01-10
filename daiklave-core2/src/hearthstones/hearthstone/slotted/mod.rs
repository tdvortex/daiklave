use crate::hearthstones::HearthstoneId;

use super::{details::HearthstoneDetails, origin::HearthstoneOrigin};

mod memo;

pub(crate) struct SlottedHearthstone<'source> {
    hearthstone_id: HearthstoneId,
    details: HearthstoneDetails<'source>,
    origin: HearthstoneOrigin<'source>,
}