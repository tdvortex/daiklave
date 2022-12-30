use std::{ops::Deref, collections::{HashSet, HashMap}};

use crate::{id::UniqueId, book_reference::BookReference, weapons::WeaponId, armor::ArmorWeight, abilities::{Ability, AbilityView}};

/// A unique identifier for a Martial Arts style.
pub struct MartialArtsStyleId(pub UniqueId);

impl Deref for MartialArtsStyleId {
    type Target = UniqueId;

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

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.usable_weapons.iter().copied()
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        self.max_armor_weight
    }

    pub fn as_view(&self) -> MartialArtsStyleView {
        MartialArtsStyleView { 
            book_reference: self.book_reference(),
            name: self.name(),
            description: self.description(),
            usable_weapons: self.usable_weapon_ids().collect(),
            max_armor_weight: self.max_armor_weight(),
        }
    }
}

pub struct MartialArtsStyleView<'source> {
    book_reference: Option<BookReference>,
    name: &'source str,
    description: &'source str,
    usable_weapons: HashSet<WeaponId>,
    max_armor_weight: Option<ArmorWeight>,
}

pub(crate) struct MortalMartialArtist {
    style: MartialArtsStyle,
    ability: Ability,
}

pub(crate) struct MortalMartialArtistView<'source> {
    style: MartialArtsStyleView<'source>,
    ability: AbilityView<'source>,
}

pub(crate) struct ExaltMartialArtist {
    style: MartialArtsStyle,
    ability: Ability,
    charms: HashMap<MartialArtsCharmId, MartialArtsCharm>,
}

pub(crate) struct ExaltMartialArtistView<'source> {
    style: MartialArtsStyleView<'source>,
    ability: AbilityView<'source>,
    charms: HashMap<MartialArtsCharmId, MartialArtsCharmView<'source>>,
}

/// A unique identifier for a Martial Arts Charm.
pub struct MartialArtsCharmId(pub UniqueId);

impl Deref for MartialArtsCharmId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct MartialArtsCharm {

}