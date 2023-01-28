mod alias;
mod merit;
mod name;
pub use alias::AddSorceryArchetype;
pub use merit::{SorceryArchetypeMerit, SorceryArchetypeMeritId};
pub use name::SorceryArchetypeName;

mod shaping_ritual;
pub use shaping_ritual::{AddShapingRitual, ShapingRitual, ShapingRitualSummary};

mod with_merits;
pub use with_merits::SorceryArchetypeWithMerits;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

/// A sorcery archetype, representing one path to sorcerous knowledge. This
/// unlocks various shaping rituals as well as unique merits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SorceryArchetype {
    book_reference: Option<BookReference>,
    description: String,
}

impl SorceryArchetype {
    /// Creates a new SorceryArchetype.
    pub fn new(
        name: String,
        book_reference: Option<BookReference>,
        description: String,
    ) -> AddSorceryArchetype {
        (
            name,
            Self {
                book_reference,
                description,
            },
        )
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
