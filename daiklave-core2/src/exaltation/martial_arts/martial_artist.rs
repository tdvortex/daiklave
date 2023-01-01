use crate::{
    armor::ArmorWeight,
    book_reference::BookReference,
    exaltation::{
        exalt::martial_arts::ExaltMartialArtist, mortal::martial_arts::MortalMartialArtist,
    },
    martial_arts::{MartialArtsCharm, MartialArtsCharmId},
    weapons::WeaponId,
};

pub(crate) enum ExaltationMartialArtist<'char> {
    Mortal(&'char MortalMartialArtist),
    Exalt(&'char ExaltMartialArtist),
}

impl<'char> ExaltationMartialArtist<'char> {
    pub fn name(&self) -> &'char str {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style.name(),
            ExaltationMartialArtist::Exalt(view) => view.style.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style.book_reference(),
            ExaltationMartialArtist::Exalt(view) => view.style.book_reference(),
        }
    }

    pub fn description(&self) -> &'char str {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style.description(),
            ExaltationMartialArtist::Exalt(view) => view.style.description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style.usable_weapon_ids(),
            ExaltationMartialArtist::Exalt(view) => view.style.usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style.max_armor_weight(),
            ExaltationMartialArtist::Exalt(view) => view.style.max_armor_weight(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.ability.dots(),
            ExaltationMartialArtist::Exalt(view) => view.ability.dots(),
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'char str> {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.ability.specialties(),
            ExaltationMartialArtist::Exalt(view) => view.ability.specialties(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'char MartialArtsCharm)> + '_ {
        match self {
            ExaltationMartialArtist::Mortal(_) => Vec::new().into_iter(),
            ExaltationMartialArtist::Exalt(view) => view
                .charms
                .iter()
                .map(|(k, v)| (*k, v))
                .collect::<Vec<(MartialArtsCharmId, &'char MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}
