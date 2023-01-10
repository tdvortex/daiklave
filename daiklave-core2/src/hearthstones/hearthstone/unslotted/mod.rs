use super::{details::HearthstoneDetails, origin::HearthstoneOrigin};

pub(crate) struct UnslottedHearthstone<'source> {
    details: HearthstoneDetails<'source>,
    origin: HearthstoneOrigin<'source>,
}