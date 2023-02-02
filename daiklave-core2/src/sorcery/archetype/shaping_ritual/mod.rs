mod add;
mod details;
pub(crate) use details::ShapingRitualDetails;
pub use add::{AddShapingRitual};

use crate::book_reference::BookReference;

/// A shaping ritual used by sorcerers to accrue sorcerous motes to fuel their
/// spellcasting.
pub struct ShapingRitual<'source> {
    pub(crate) archetype_name: &'source str,
    pub(crate) summary: &'source str,
    pub(crate) details: &'source ShapingRitualDetails
}

impl<'source> ShapingRitual<'source> {
    /// The name of the sorcery archetype associated with the ritual.
    pub fn archetype_name(&self) -> &'source str {
        self.archetype_name
    }

    /// A short summary of the ritual's effect.
    pub fn summary(&self) -> &'source str {
        self.summary
    }

    /// A page reference for the ritual, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.details.book_reference
    }

    /// A description of the ritual.
    pub fn description(&self) -> &'source str {
        &self.details.description
    }
}