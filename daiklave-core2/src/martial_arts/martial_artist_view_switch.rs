use crate::{armor::ArmorWeight, book_reference::BookReference, weapons::WeaponId};

use super::{
    charm::MartialArtsCharm, charm_id::MartialArtsCharmId,
    exalt_martial_artist_view::ExaltMartialArtistView,
    mortal_martial_artist_view::MortalMartialArtistView,
};

pub(crate) enum MartialArtistViewSwitch<'view, 'source> {
    Mortal(&'view MortalMartialArtistView<'source>),
    Exalt(&'view ExaltMartialArtistView<'source>),
}

impl<'view, 'source> MartialArtistViewSwitch<'view, 'source> {
    pub fn name(&self) -> &'source str {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.name(),
            MartialArtistViewSwitch::Exalt(view) => view.style.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.book_reference(),
            MartialArtistViewSwitch::Exalt(view) => view.style.book_reference(),
        }
    }

    pub fn description(&self) -> &'source str {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.description(),
            MartialArtistViewSwitch::Exalt(view) => view.style.description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.usable_weapon_ids(),
            MartialArtistViewSwitch::Exalt(view) => view.style.usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.max_armor_weight(),
            MartialArtistViewSwitch::Exalt(view) => view.style.max_armor_weight(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.ability.dots(),
            MartialArtistViewSwitch::Exalt(view) => view.ability.dots(),
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'source str> {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.ability.specialties(),
            MartialArtistViewSwitch::Exalt(view) => view.ability.specialties(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
        match self {
            MartialArtistViewSwitch::Mortal(_) => Vec::new().into_iter(),
            MartialArtistViewSwitch::Exalt(view) => view
                .charms
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<(MartialArtsCharmId, &'source MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}
