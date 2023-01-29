mod add;
mod details;
pub(crate) use details::ShapingRitualDetails;
pub use add::{AddShapingRitual};

use crate::book_reference::BookReference;



pub struct ShapingRitual<'source> {
    archetype_name: &'source str,
    summary: &'source str,
    details: &'source ShapingRitualDetails
}

impl<'source> ShapingRitual<'source> {
    pub fn archetype_name(&self) -> &'source str {
        self.archetype_name
    }

    pub fn summary(&self) -> &'source str {
        self.summary
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.details.book_reference
    }

    pub fn description(&self) -> &'source str {
        &self.details.description
    }
}