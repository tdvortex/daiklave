use std::collections::HashSet;

use crate::{
    armor::armor_item::ArmorWeightClass,
    book_reference::BookReference,
    martial_arts::style::{MartialArtsStyleDetails, MartialArtsStyleWeapon, MartialArtsStyleName, AddMartialArtsStyle},
};

/// A Martial Arts style builder after at least one weapon has been specified.
/// To complete the builder, call build().
pub struct MartialArtsStyleBuilderWithWeapons {
    pub(crate) name: MartialArtsStyleName,
    pub(crate) description: String,
    pub(crate) first_weapon: MartialArtsStyleWeapon,
    pub(crate) usable_weapons: HashSet<MartialArtsStyleWeapon>,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) max_armor_weight: Option<ArmorWeightClass>,
}

impl MartialArtsStyleBuilderWithWeapons {
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
    pub fn unarmed(mut self) -> Self {
        if !matches!(self.first_weapon, MartialArtsStyleWeapon::Unarmed) {
            self.usable_weapons.insert(MartialArtsStyleWeapon::Unarmed);
        }
        self
    }

    /// Enables the style to be used with a specific type of weapon. This may
    /// be a mundane weapon (like "sword"), a category of artifact weapon (like
    /// "dailklave"), but not a specific artifact weapon (like "Spring Razor").
    pub fn weapon(mut self, weapon: String) -> Self {
        let style_weapon = MartialArtsStyleWeapon::BaseWeapon(weapon);

        if self.first_weapon != style_weapon {
            self.usable_weapons.insert(style_weapon);
        }
        self
    }

    /// Completes the builder, returning a Martial Arts style.
    pub fn build(self) -> AddMartialArtsStyle {
        AddMartialArtsStyle {
            style_name: self.name,
            style: MartialArtsStyleDetails {
                book_reference: self.book_reference,
                description: self.description,
                first_weapon: self.first_weapon,
                usable_weapons: self.usable_weapons,
                max_armor_weight: self.max_armor_weight,
            },
        }
    }
}
