use crate::{
    armor::ArmorWeight,
    book_reference::BookReference,
    exaltation::{
        exalt::martial_arts::ExaltMartialArtistMemo, mortal::martial_arts::MortalMartialArtist,
    },
    martial_arts::{MartialArtsCharm, MartialArtsCharmId},
    weapons::WeaponId,
};

pub(crate) enum ExaltationMartialArtistMemo<'char> {
    Mortal(&'char MortalMartialArtist),
    Exalt(&'char ExaltMartialArtistMemo),
}

impl<'char> ExaltationMartialArtistMemo<'char> {
    pub fn name(&self) -> &'char str {
        match self {
            ExaltationMartialArtistMemo::Mortal(view) => view.style.name(),
            ExaltationMartialArtistMemo::Exalt(view) => view.style.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            ExaltationMartialArtistMemo::Mortal(view) => view.style.book_reference(),
            ExaltationMartialArtistMemo::Exalt(view) => view.style.book_reference(),
        }
    }

    pub fn description(&self) -> &'char str {
        match self {
            ExaltationMartialArtistMemo::Mortal(view) => view.style.description(),
            ExaltationMartialArtistMemo::Exalt(view) => view.style.description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            ExaltationMartialArtistMemo::Mortal(view) => view.style.usable_weapon_ids(),
            ExaltationMartialArtistMemo::Exalt(view) => view.style.usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            ExaltationMartialArtistMemo::Mortal(view) => view.style.max_armor_weight(),
            ExaltationMartialArtistMemo::Exalt(view) => view.style.max_armor_weight(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            ExaltationMartialArtistMemo::Mortal(view) => view.ability.dots(),
            ExaltationMartialArtistMemo::Exalt(view) => view.ability.dots(),
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'char str> {
        match self {
            ExaltationMartialArtistMemo::Mortal(view) => view.ability.specialties(),
            ExaltationMartialArtistMemo::Exalt(view) => view.ability.specialties(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'char MartialArtsCharm)> + '_ {
        match self {
            ExaltationMartialArtistMemo::Mortal(_) => Vec::new().into_iter(),
            ExaltationMartialArtistMemo::Exalt(view) => view
                .charms
                .iter()
                .map(|(k, v)| (*k, v))
                .collect::<Vec<(MartialArtsCharmId, &'char MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}
