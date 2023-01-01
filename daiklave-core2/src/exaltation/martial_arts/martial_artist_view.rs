use crate::{
    armor::ArmorWeight,
    book_reference::BookReference,
    exaltation::{
        exalt::martial_arts::ExaltMartialArtistView, mortal::martial_arts::MortalMartialArtistView,
    },
    martial_arts::{MartialArtsCharm, MartialArtsCharmId},
    weapons::WeaponId,
};

pub(crate) enum ExaltationMartialArtistView<'view, 'source> {
    Mortal(&'view MortalMartialArtistView<'source>),
    Exalt(&'view ExaltMartialArtistView<'source>),
}

impl<'view, 'source> ExaltationMartialArtistView<'view, 'source> {
    pub fn name(&self) -> &'source str {
        match self {
            ExaltationMartialArtistView::Mortal(view) => view.style().name(),
            ExaltationMartialArtistView::Exalt(view) => view.style().name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            ExaltationMartialArtistView::Mortal(view) => view.style().book_reference(),
            ExaltationMartialArtistView::Exalt(view) => view.style().book_reference(),
        }
    }

    pub fn description(&self) -> &'source str {
        match self {
            ExaltationMartialArtistView::Mortal(view) => view.style().description(),
            ExaltationMartialArtistView::Exalt(view) => view.style().description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            ExaltationMartialArtistView::Mortal(view) => view.style().usable_weapon_ids(),
            ExaltationMartialArtistView::Exalt(view) => view.style().usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            ExaltationMartialArtistView::Mortal(view) => view.style().max_armor_weight(),
            ExaltationMartialArtistView::Exalt(view) => view.style().max_armor_weight(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            ExaltationMartialArtistView::Mortal(view) => view.ability().dots(),
            ExaltationMartialArtistView::Exalt(view) => view.ability().dots(),
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'source str> {
        match self {
            ExaltationMartialArtistView::Mortal(view) => view.ability().specialties(),
            ExaltationMartialArtistView::Exalt(view) => view.ability().specialties(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
        match self {
            ExaltationMartialArtistView::Mortal(_) => Vec::new().into_iter(),
            ExaltationMartialArtistView::Exalt(view) => view
                .charms()
                .collect::<Vec<(MartialArtsCharmId, &'source MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}
