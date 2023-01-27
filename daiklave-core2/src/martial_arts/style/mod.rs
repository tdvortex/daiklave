/// Builder path for constructing a new Martial Arts style.
pub mod builder;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{armor::armor_item::ArmorWeightClass, book_reference::BookReference};

use self::builder::MartialArtsStyleBuilder;

/// A Martial Arts style description.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MartialArtsStyle {
    book_reference: Option<BookReference>,
    description: String,
    first_weapon: String,
    usable_weapons: HashSet<String>,
    max_armor_weight: Option<ArmorWeightClass>,
}

impl<'source> MartialArtsStyle {
    /// Starts a builder to construct a new Martial Arts style.
    pub fn builder(name: String) -> MartialArtsStyleBuilder {
        MartialArtsStyleBuilder {
            name,
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

    /// A list of weapon names, which may be either mortal weapons (e.g. sword)
    /// or base artifact weapons (e.g. daiklave), usable by the style.
    pub fn usable_weapon_names(&'source self) -> impl Iterator<Item = &'source str> + '_ {
        std::iter::once(self.first_weapon.as_str())
            .chain(self.usable_weapons.iter().map(|s| s.as_str()))
    }

    /// The maximum weight of armor which may be worn with the style, or None
    /// if incompatible with armor.
    pub fn max_armor_weight(&self) -> Option<ArmorWeightClass> {
        self.max_armor_weight
    }
}
