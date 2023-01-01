use crate::{armor::ArmorWeight, book_reference::BookReference, weapons::WeaponId};

use super::{
    charm::MartialArtsCharm, charm_id::MartialArtsCharmId,
    exalt_martial_artist::ExaltMartialArtist, mortal_martial_artist::MortalMartialArtist,
};

pub(crate) enum MartialArtistSwitch<'char> {
    Mortal(&'char MortalMartialArtist),
    Exalt(&'char ExaltMartialArtist),
}

impl<'char> MartialArtistSwitch<'char> {
    pub fn name(&self) -> &'char str {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.name(),
            MartialArtistSwitch::Exalt(view) => view.style.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.book_reference(),
            MartialArtistSwitch::Exalt(view) => view.style.book_reference(),
        }
    }

    pub fn description(&self) -> &'char str {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.description(),
            MartialArtistSwitch::Exalt(view) => view.style.description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.usable_weapon_ids(),
            MartialArtistSwitch::Exalt(view) => view.style.usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.max_armor_weight(),
            MartialArtistSwitch::Exalt(view) => view.style.max_armor_weight(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            MartialArtistSwitch::Mortal(view) => view.ability.dots(),
            MartialArtistSwitch::Exalt(view) => view.ability.dots(),
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'char str> {
        match self {
            MartialArtistSwitch::Mortal(view) => view.ability.specialties(),
            MartialArtistSwitch::Exalt(view) => view.ability.specialties(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'char MartialArtsCharm)> + '_ {
        match self {
            MartialArtistSwitch::Mortal(_) => Vec::new().into_iter(),
            MartialArtistSwitch::Exalt(view) => view
                .charms
                .iter()
                .map(|(k, v)| (*k, v))
                .collect::<Vec<(MartialArtsCharmId, &'char MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}
