mod add;
mod merit;
mod name;
pub use add::AddSorceryArchetype;
pub use merit::{SorceryArchetypeMerit, SorceryArchetypeMeritName};
pub use name::SorceryArchetypeName;

mod shaping_ritual;
pub use shaping_ritual::{AddShapingRitual, ShapingRitual};

mod with_merits;
pub use with_merits::SorceryArchetypeWithMerits;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SorceryArchetype {
    book_reference: Option<BookReference>,
    description: String,
}

impl SorceryArchetype {
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}
