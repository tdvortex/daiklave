mod add;
pub mod builder;
mod details;
mod name;
pub use add::AddSorceryArchetypeMerit;
pub(crate) use details::SorceryArchetypeMeritDetails;
pub use name::SorceryArchetypeMeritName;

use crate::book_reference::BookReference;

pub struct SorceryArchetypeMerit<'source> {
    archetype_name: &'source str,
    name: &'source str,
    details: &'source SorceryArchetypeMeritDetails
}

impl<'source> SorceryArchetypeMerit<'source> {
    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn archetype_name(&self) -> &'source str {
        self.archetype_name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.details.book_reference
    }

    pub fn dots(&self) -> u8 {
        self.details.dots
    }

    pub fn description(&self) -> &'source str {
        &self.details.description
    }
}