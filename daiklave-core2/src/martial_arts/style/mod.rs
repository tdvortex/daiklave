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


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub(crate) struct MartialArtsStyleDetails {
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) description: String,
    pub(crate) first_weapon: MartialArtsStyleWeapon,
    pub(crate) usable_weapons: HashSet<MartialArtsStyleWeapon>,
    pub(crate) max_armor_weight: Option<ArmorWeightClass>,
}

impl<'source> MartialArtsStyleDetails {
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
