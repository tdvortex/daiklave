mod add;
mod details;
mod name;
mod remove;

pub use add::AddSorceryArchetypeMerit;
pub(crate) use details::SorceryArchetypeMeritDetails;
pub use name::SorceryArchetypeMeritName;
pub use remove::RemoveSorceryArchetypeMerit;

use crate::book_reference::BookReference;

pub(crate) struct SorceryArchetypeMerit<'source> {
    pub archetype_name: &'source str,
    pub merit_name: &'source str,
    pub details: &'source SorceryArchetypeMeritDetails,
}

impl<'source> SorceryArchetypeMerit<'source> {
    pub fn name(&self) -> &'source str {
        self.merit_name
    }

    pub fn detail(&self) -> Option<&'source str> {
        Some(self.archetype_name)
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