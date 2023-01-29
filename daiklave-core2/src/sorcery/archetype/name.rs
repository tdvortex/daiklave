use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{AddShapingRitual, ShapingRitualDetails};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeName(String);

impl SorceryArchetypeName {
    pub fn new_shaping_ritual(&self, 
        summary: impl ToString, 
        book_reference: Option<BookReference>, 
        description: impl ToString
    ) -> AddShapingRitual {
        AddShapingRitual {
            archetype_name: self.clone(),
            summary: summary.to_string(),
            ritual: ShapingRitualDetails {
                book_reference,
                description: description.to_string(),
            }
        }
    }

    pub fn new_merit(&self) -> SorceryArchetypeMeritBuilder {
        
    }
}

impl<T> From<T> for SorceryArchetypeName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for SorceryArchetypeName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}