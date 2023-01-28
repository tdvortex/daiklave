use crate::{
    abilities::{Ability, AbilityNameQualified}, armor::armor_item::ArmorWeightClass, book_reference::BookReference,
    exaltation::ExaltationMartialArtist,
};

use super::{charm::MartialArtsCharm, style::MartialArtsStyleWeapon};

/// A specific Martial Arts style as known by a character.
pub struct MartialArtist<'view, 'source> {
    pub(crate) name: &'source str,
    pub(crate) maybe_exalt: ExaltationMartialArtist<'view, 'source>,
}

impl<'view, 'source> MartialArtist<'view, 'source> {
    /// The style's name.
    pub fn name(&self) -> &'source str {
        self.name
    }

    /// The book reference for the style.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.maybe_exalt.book_reference()
    }

    /// The style's description.
    pub fn description(&self) -> &'source str {
        self.maybe_exalt.description()
    }

    /// All of the base weapons usable by the style. This is either Unarmed or
    /// a base weapon (e.g. "sword" or "daiklave"), not any specific unique
    /// artifact weapon.
    pub fn usable_weapons(&self) -> impl Iterator<Item = &'source MartialArtsStyleWeapon> + '_ {
        self.maybe_exalt.usable_weapons()
    }

    /// If the style is usable with armor, gives the heaviest weight category
    /// allowed.
    pub fn max_armor_weight(&self) -> Option<ArmorWeightClass> {
        self.maybe_exalt.max_armor_weight()
    }

    /// The details of the Martial Arts ability the character has for this
    /// style, including rating and specialties.
    pub fn ability(&'view self) -> Ability<'view, 'source> {
        Ability (
            AbilityNameQualified::MartialArts(self.name),
            self.maybe_exalt.ability_rating()
        )
    }

    /// All of the Charms the character has for this style.
    pub fn charms(&self) -> impl Iterator<Item = (&'source str, &'source MartialArtsCharm)> + '_ {
        self.maybe_exalt.charms()
    }
}
