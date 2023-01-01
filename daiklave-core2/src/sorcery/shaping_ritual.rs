use serde::{Serialize, Deserialize};

use crate::book_reference::BookReference;

use super::archetype_id::SorceryArchetypeId;

/// A shaping ritual, one method that a sorcerous archetype might use to
/// generate Sorcerous Motes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapingRitual {
    archetype_id: SorceryArchetypeId,
    book_reference: Option<BookReference>,
    description: String,
}

impl ShapingRitual {
    /// Create a new ShapingRitual
    pub fn new(
        archetype_id: SorceryArchetypeId,
        book_reference: Option<BookReference>,
        description: String,
    ) -> Self {
        Self {
            archetype_id,
            book_reference,
            description,
        }
    }

    /// The Id of the SorceryArchetype associated with this ritual
    pub fn archetype_id(&self) -> SorceryArchetypeId {
        self.archetype_id
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