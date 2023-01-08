use crate::{
    abilities::Ability, armor::armor_item::ArmorWeightClass, book_reference::BookReference,
    exaltation::ExaltationMartialArtist, weapons::weapon::BaseWeaponId,
};

use super::{charm::MartialArtsCharm, charm_id::MartialArtsCharmId, MartialArtsStyleId};

use crate::abilities::AbilityType;

/// A specific Martial Arts style as known by a character.
pub struct MartialArtist<'view, 'source> {
    id: MartialArtsStyleId,
    maybe_exalt: ExaltationMartialArtist<'view, 'source>,
}

impl<'view, 'source> MartialArtist<'view, 'source> {
    pub(crate) fn new(
        id: MartialArtsStyleId,
        maybe_exalt: ExaltationMartialArtist<'view, 'source>,
    ) -> Self {
        Self { id, maybe_exalt }
    }

    /// The style's name.
    pub fn name(&self) -> &'source str {
        self.maybe_exalt.name()
    }

    /// The book reference for the style.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.maybe_exalt.book_reference()
    }

    /// The style's description.
    pub fn description(&self) -> &'source str {
        self.maybe_exalt.description()
    }

    /// All of the base weapon Ids usable by the style. This is the base weapon
    /// (e.g. "sword" or "daiklave"), not any specific unique artifact weapon.
    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = BaseWeaponId> + '_ {
        self.maybe_exalt.usable_weapon_ids()
    }

    /// If the style is usable with armor, gives the heaviest weight category
    /// allowed.
    pub fn max_armor_weight(&self) -> Option<ArmorWeightClass> {
        self.maybe_exalt.max_armor_weight()
    }

    /// The details of the Martial Arts ability the character has for this
    /// style, including rating and specialties.
    pub fn ability(&'view self) -> Ability<'view, 'source> {
        Ability(AbilityType::MartialArts(
            self.id,
            self.maybe_exalt.ability_rating(),
        ))
    }

    /// All of the Charms the character has for this style.
    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
        self.maybe_exalt.charms()
    }
}
