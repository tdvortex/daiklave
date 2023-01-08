pub mod builder;
mod id;

use std::collections::HashSet;

pub use id::BaseArmorId;
use serde::{Serialize, Deserialize};

use crate::book_reference::BookReference;

use super::{ArmorWeightClass, ArmorTag};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BaseArmor {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub weight_class: ArmorWeightClass,
    pub tags: HashSet<ArmorTag>,
}