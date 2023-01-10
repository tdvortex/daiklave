mod memo;
pub(crate) use memo::HearthstoneDetailsMemo;

use crate::{book_reference::BookReference, hearthstones::{category::HearthstoneCategory, geomancy_level::GeomancyLevel}};

pub(crate) struct HearthstoneDetails<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    category: HearthstoneCategory,
    geomancy_level: GeomancyLevel,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
    is_dependent: bool,
}