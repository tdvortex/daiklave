use crate::{
    abilities::AbilityRating,
    armor::armor_item::ArmorWeightClass,
    book_reference::BookReference,
    exaltation::{
        exalt::martial_arts::ExaltMartialArtistDetails, mortal::martial_arts::MortalMartialArtistDetails,
    },
    martial_arts::{charm::{MartialArtsCharm}, style::MartialArtsStyleWeapon},
};

pub(crate) enum ExaltationMartialArtist<'view, 'source> {
    Mortal(&'view MortalMartialArtistDetails<'source>),
    Exalt(&'view ExaltMartialArtistDetails<'source>),
}

impl<'view, 'source> ExaltationMartialArtist<'view, 'source> {
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

    pub fn usable_weapons(&self) -> impl Iterator<Item = &'source MartialArtsStyleWeapon> + '_ {
        match self {
            ExaltationMartialArtist::Mortal(view) => view.style().usable_weapons(),
            ExaltationMartialArtist::Exalt(view) => view.style().usable_weapons(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeightClass> {
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

    pub fn charms(&self, style_name: &'source str) -> impl Iterator<Item = MartialArtsCharm<'source>> + '_ {
        match self {
            ExaltationMartialArtist::Mortal(_) => vec![].into_iter(),
            ExaltationMartialArtist::Exalt(view) => view
                .charms(style_name)
                .collect::<Vec<MartialArtsCharm<'source>>>()
                .into_iter(),
        }
    }
}
