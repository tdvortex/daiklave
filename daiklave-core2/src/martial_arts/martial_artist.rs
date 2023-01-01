use crate::{armor::ArmorWeight, book_reference::BookReference, weapons::WeaponId};

use super::{
    charm::MartialArtsCharm, charm_id::MartialArtsCharmId,
    martial_artist_switch::MartialArtistSwitch,
};

/// A specific Martial Arts style as known by a character.
pub struct MartialArtist<'char>(pub(crate) MartialArtistSwitch<'char>);

impl<'char> MartialArtist<'char> {
    /// The style's name.
    pub fn name(&self) -> &'char str {
        self.0.name()
    }

    /// The book reference for the style.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// The style's description.
    pub fn description(&self) -> &'char str {
        self.0.description()
    }

    /// All of the base weapon Ids usable by the style. This is the base weapon
    /// (e.g. "sword" or "daiklave"), not any specific unique artifact weapon.
    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.0.usable_weapon_ids()
    }

    /// If the style is usable with armor, gives the heaviest weight category
    /// allowed.
    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        self.0.max_armor_weight()
    }

    /// The number of ability dots the character possesses in the skill.
    pub fn dots(&self) -> u8 {
        self.0.dots()
    }

    /// Any specialties the character has in this Martial Arts style.
    pub fn specialties(&self) -> impl Iterator<Item = &'char str> {
        self.0.specialties()
    }

    /// All of the Charms the character has for this style.
    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'char MartialArtsCharm)> + '_ {
        self.0.charms()
    }
}
