mod base_armor_item_builder;
mod with_weight_class;

use std::collections::HashSet;

pub use base_armor_item_builder::BaseArmorItemBuilder;
use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, armor::armor_item::{ArmorWeightClass, ArmorTag}};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BaseArmor {
    pub name: String,
    pub book_reference: Option<BookReference>,
    pub weight_class: ArmorWeightClass,
    pub tags: HashSet<ArmorTag>,
}