/// Builder path for constructing a new Martial Arts style.
pub mod builder;
mod add;
mod name;
mod weapon;
pub use add::AddMartialArtsStyle;
pub use name::MartialArtsStyleName;
pub use weapon::MartialArtsStyleWeapon;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{armor::armor_item::ArmorWeightClass, book_reference::BookReference};

use self::builder::MartialArtsStyleBuilder;

/// A Martial Arts style description.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MartialArtsStyle {
    book_reference: Option<BookReference>,
    description: String,
    first_weapon: MartialArtsStyleWeapon,
    usable_weapons: HashSet<MartialArtsStyleWeapon>,
    max_armor_weight: Option<ArmorWeightClass>,
}

impl<'source> MartialArtsStyle {
    /// Starts a builder to construct a new Martial Arts style.
    pub fn with_name(name: impl Into<MartialArtsStyleName>) -> MartialArtsStyleBuilder {
        MartialArtsStyleBuilder {
            name: name.into(),
            book_reference: None,
            max_armor_weight: None,
        }
    }

    /// The page reference for the style (if any).
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The style's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// A list of weapons, which may be either mortal weapons (e.g. sword)
    /// or base artifact weapons (e.g. daiklave), usable by the style.
    pub fn usable_weapons(
        &'source self,
    ) -> impl Iterator<Item = &'source MartialArtsStyleWeapon> + '_ {
        std::iter::once(&self.first_weapon).chain(self.usable_weapons.iter())
    }

    /// The maximum weight of armor which may be worn with the style, or None
    /// if incompatible with armor.
    pub fn max_armor_weight(&self) -> Option<ArmorWeightClass> {
        self.max_armor_weight
    }
}
