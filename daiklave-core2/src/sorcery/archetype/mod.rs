mod add;
mod merit;
mod merits;
mod name;
pub use add::AddSorceryArchetype;
pub(crate) use merit::SorceryArchetypeMeritDetails;
pub use merit::{SorceryArchetypeMeritName, SorceryArchetypeMerit};
pub use merits::SorceryArchetypeMerits;
pub use name::SorceryArchetypeName;

mod shaping_ritual;
pub(crate) use shaping_ritual::ShapingRitualDetails;
pub use shaping_ritual::{AddShapingRitual};

mod with_merits;
pub use with_merits::SorceryArchetypeWithMerits;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SorceryArchetypeDetails {
    pub book_reference: Option<BookReference>,
    pub description: String,
}