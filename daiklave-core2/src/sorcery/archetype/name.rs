use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{AddShapingRitual, ShapingRitualDetails};

/// The name of a sorcery archetype.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeName(String);

impl SorceryArchetypeName {
    /// Create a new shaping ritual associated with this archetype.
    pub fn new_shaping_ritual(
        &self,
        summary: impl Into<String>,
        book_reference: Option<BookReference>,
        description: impl Into<String>,
    ) -> AddShapingRitual {
        AddShapingRitual {
            archetype_name: self.clone(),
            summary: summary.into(),
            ritual: ShapingRitualDetails {
                book_reference,
                description: description.into(),
            },
        }
    }
}

impl<T> From<T> for SorceryArchetypeName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for SorceryArchetypeName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
