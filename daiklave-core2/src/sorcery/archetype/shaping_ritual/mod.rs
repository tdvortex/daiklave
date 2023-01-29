mod alias;
pub use alias::{AddShapingRitual};

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

/// A shaping ritual, one method that a sorcerous archetype might use to
/// generate Sorcerous Motes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapingRitual {
    book_reference: Option<BookReference>,
    description: String,
}

impl ShapingRitual {
    /// The book reference for the shaping ritual, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// A description of the shaping ritual
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}
