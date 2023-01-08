use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    armor::armor_item::ArmorWeightClass, book_reference::BookReference,
    weapons::weapon::BaseWeaponId,
};

/// A Martial Arts style description.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MartialArtsStyle {
    book_reference: Option<BookReference>,
    name: String,
    description: String,
    usable_weapons: HashSet<BaseWeaponId>,
    max_armor_weight: Option<ArmorWeightClass>,
}

impl MartialArtsStyle {
    /// Construct a new Martial Arts style
    pub fn new(
        book_reference: Option<BookReference>,
        name: String,
        description: String,
        usable_weapons: HashSet<BaseWeaponId>,
        max_armor_weight: Option<ArmorWeightClass>,
    ) -> Self {
        Self {
            book_reference,
            name,
            description,
            usable_weapons,
            max_armor_weight,
        }
    }

    /// The page reference for the style (if any).
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The style's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The style's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// A list of weapon ids, which may be either mortal weapons (e.g. sword)
    /// or base artifact weapons (e.g. daiklave), usable by the style.
    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = BaseWeaponId> + '_ {
        self.usable_weapons.iter().copied()
    }

    /// The maximum weight of armor which may be worn with the style, or None
    /// if incompatible with armor.
    pub fn max_armor_weight(&self) -> Option<ArmorWeightClass> {
        self.max_armor_weight
    }
}
