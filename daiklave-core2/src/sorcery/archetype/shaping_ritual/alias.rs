use crate::{sorcery::archetype::SorceryArchetypeName, book_reference::BookReference};

use super::ShapingRitual;

/// A shaping ritual to add to a character.
pub struct AddShapingRitual {
    pub(crate) archetype_name: SorceryArchetypeName,
    pub(crate) summary: String,
    pub(crate) ritual: ShapingRitual
}

impl AddShapingRitual {
    /// Create a new ShapingRitual
    pub fn new(
        archetype_name: SorceryArchetypeName,
        summary: String,
        book_reference: Option<BookReference>,
        description: String,
    ) -> Self {
        Self {
            archetype_name,
            summary,
            ritual: ShapingRitual { book_reference, description}
        }
    }
}