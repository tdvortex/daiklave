use std::{ops::Deref, collections::HashSet};

use crate::{id::Id, book_reference::BookReference, weapons::WeaponId, armor::ArmorWeight};

/// A unique identifier for a Martial Arts style.
pub struct MartialArtsStyleId(pub Id);

impl Deref for MartialArtsStyleId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Martial Arts style description. 
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MartialArtsStyle {
    book_reference: Option<BookReference>,
    name: String,
    description: String,
    usable_weapons: HashSet<WeaponId>,
    max_armor_weight: Option<ArmorWeight>,
}

impl MartialArtsStyle {
    /// Construct a new Martial Arts style
    pub fn new(
        book_reference: Option<BookReference>,
        name: String,
        description: String,
        usable_weapons: HashSet<WeaponId>,
        max_armor_weight: Option<ArmorWeight>,
    ) -> Self {
        Self {
            book_reference,
            name,
            description,
            usable_weapons,
            max_armor_weight,
        }
    }
}