mod add;
pub use add::AddSorceryArchetypeMerit;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

/// A merit which is made available to sorcerers following a specific
/// Archetype. These are always inherently nonstackable, fixed-dot, and have no
/// prerequisites apart from the archetype itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SorceryArchetypeMerit {
    book_reference: Option<BookReference>,
    dots: u8,
    description: String,
}

impl<'source> SorceryArchetypeMerit {
    /// Creates a new Sorcery Archetype merit.
    pub fn new(
        sorcery_archetype_name: String,
        merit_name: String,
        book_reference: Option<BookReference>,
        dots: u8,
        description: String,
    ) -> AddSorceryArchetypeMerit {
        (sorcery_archetype_name,
            merit_name, 
        SorceryArchetypeMerit {
            book_reference,
            dots,
            description,
        })
    }

    /// The name of the merit.
    pub fn name(&'source self) -> &'source str {
        self.name.as_str()
    }

    /// The book reference for the merit.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The cost of the merit in dots.
    pub fn dots(&self) -> u8 {
        self.dots
    }

    /// The merit's description.
    pub fn description(&'source self) -> &'source str {
        self.description.as_str()
    }
}
