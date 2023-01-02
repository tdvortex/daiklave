use std::collections::HashSet;

use crate::book_reference::BookReference;

use self::{geomancy_level::GeomancyLevel, category::HearthstoneCategory, keyword::HearthstoneKeyword};

mod category;
mod geomancy_level;
mod keyword;
mod owned;


pub(in crate::weapons) use owned::OwnedHearthstone;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) struct Hearthstone<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    geomancy_level: GeomancyLevel,
    category: HearthstoneCategory,
    keywords: HashSet<HearthstoneKeyword>,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
}