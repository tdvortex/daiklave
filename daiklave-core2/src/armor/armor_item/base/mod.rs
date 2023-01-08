pub mod builder;
mod id;

use std::collections::HashSet;

pub use id::BaseArmorId;
use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{ArmorTag, ArmorWeightClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BaseArmor {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub weight_class: ArmorWeightClass,
    pub tags: HashSet<ArmorTag>,
}

impl BaseArmor {
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn weight_class(&self) -> ArmorWeightClass {
        self.weight_class
    }

    pub fn tags(&self) -> impl Iterator<Item = ArmorTag> + '_ {
        self.tags.iter().copied()
    }
}
