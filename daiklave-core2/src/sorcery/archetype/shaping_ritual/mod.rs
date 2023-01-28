mod alias;
pub use alias::{AddShapingRitual, ShapingRitualSummary};

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::SorceryArchetypeName;

/// A shaping ritual, one method that a sorcerous archetype might use to
/// generate Sorcerous Motes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapingRitual {
    book_reference: Option<BookReference>,
    description: String,
}

impl ShapingRitual {
    /// Create a new ShapingRitual
    pub fn new(
        archetype_name: SorceryArchetypeName,
        ritual_summary: ShapingRitualSummary,
        book_reference: Option<BookReference>,
        description: String,
    ) -> AddShapingRitual {
        (
            archetype_name,
            ritual_summary,
            Self {
                book_reference,
                description,
            },
        )
    }

    /// The book reference for the shaping ritual, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// A description of the shaping ritual
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}
