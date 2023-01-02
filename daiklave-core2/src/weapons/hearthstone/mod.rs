use crate::book_reference::BookReference;

use self::geomancy_level::GeomancyLevel;

mod geomancy_level;
mod owned;

pub(in crate::weapons) struct Hearthstone<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    geomancy_level: GeomancyLevel,
    lore: Option<String>,
    powers: Option<String>,
}

pub(in crate::weapons) use owned::OwnedHearthstone;