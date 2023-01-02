use crate::{
    abilities::AbilityRating,
    armor::ArmorWeight,
    book_reference::BookReference,
    exaltation::{
        exalt::martial_arts::ExaltMartialArtist, mortal::martial_arts::MortalMartialArtistView,
    },
    martial_arts::{MartialArtsCharm, MartialArtsCharmId},
    weapons::WeaponId,
};

pub(crate) enum ExaltationMartialArtist<'view, 'source> {
    Mortal(&'view MortalMartialArtistView<'source>),
    Exalt(&'view ExaltMartialArtist<'source>),
}

impl<'view, 'source> ExaltationMartialArtist<'view, 'source> {
    pub fn name(&self) -> &'source str {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style().name(),
            ExaltationMartialArtist::Exalt(view) => view.style().name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style().book_reference(),
            ExaltationMartialArtist::Exalt(view) => view.style().book_reference(),
        }
    }

    pub fn description(&self) -> &'source str {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style().description(),
            ExaltationMartialArtist::Exalt(view) => view.style().description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style().usable_weapon_ids(),
            ExaltationMartialArtist::Exalt(view) => view.style().usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style().max_armor_weight(),
            ExaltationMartialArtist::Exalt(view) => view.style().max_armor_weight(),
        }
    }

    pub fn ability_rating(&self) -> &'view AbilityRating<'source> {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.ability(),
            ExaltationMartialArtist::Exalt(view) => view.ability(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
        match self {
            ExaltationMartialArtist::Mortal(_) => Vec::new().into_iter(),
            ExaltationMartialArtist::Exalt(view) => view
                .charms()
                .collect::<Vec<(MartialArtsCharmId, &'source MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}
