use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

/// A sorcery archetype, representing one path to sorcerous knowledge. This
/// unlocks various shaping rituals as well as unique merits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SorceryArchetype {
    name: String,
    book_reference: Option<BookReference>,
    description: String,
}

impl SorceryArchetype {
    /// Creates a new SorceryArchetype.
    pub fn new(name: String, book_reference: Option<BookReference>, description: String) -> Self {
        Self {
            name,
            book_reference,
            description,
        }
    }

    /// The name of the archetype
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The book reference for the archetype, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// A description of the archetype
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}
