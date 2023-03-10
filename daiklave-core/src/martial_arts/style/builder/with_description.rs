use std::collections::HashSet;

use crate::{
    armor::armor_item::ArmorWeightClass,
    book_reference::BookReference,
    martial_arts::style::{MartialArtsStyleName, MartialArtsStyleWeapon},
};

use super::MartialArtsStyleBuilderWithWeapons;

/// A Martial Arts Style builder after the description has been specified.
pub struct MartialArtsStyleBuilderWithDescription {
    pub(crate) name: MartialArtsStyleName,
    pub(crate) description: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) max_armor_weight: Option<ArmorWeightClass>,
}

impl MartialArtsStyleBuilderWithDescription {
    /// The book reference for the style.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// If the style is compatible with armor, the maximum usable weight.
    pub fn max_armor_weight(mut self, weight: ArmorWeightClass) -> Self {
        self.max_armor_weight = Some(weight);
        self
    }

    /// Allows the style to be used unarmed.
    pub fn unarmed(self) -> MartialArtsStyleBuilderWithWeapons {
        MartialArtsStyleBuilderWithWeapons {
            name: self.name,
            description: self.description,
            first_weapon: MartialArtsStyleWeapon::Unarmed,
            book_reference: self.book_reference,
            usable_weapons: HashSet::new(),
            max_armor_weight: self.max_armor_weight,
        }
    }

    /// Enables the style to be used with a specific type of weapon. This may
    /// be a mundane weapon (like "sword"), a category of artifact weapon (like
    /// "dailklave"), but not a specific artifact weapon (like "Spring Razor").
    pub fn weapon(self, weapon: impl Into<String>) -> MartialArtsStyleBuilderWithWeapons {
        MartialArtsStyleBuilderWithWeapons {
            name: self.name,
            description: self.description,
            first_weapon: MartialArtsStyleWeapon::BaseWeapon(weapon.into()),
            book_reference: self.book_reference,
            usable_weapons: HashSet::new(),
            max_armor_weight: self.max_armor_weight,
        }
    }
}
