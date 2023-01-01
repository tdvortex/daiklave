use crate::abilities::AbilityName;

use super::{
    caste_memo::SolarCasteMemo, dawn::DawnView, eclipse::EclipseView, night::NightView,
    twilight::TwilightView, zenith::ZenithView,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolarCasteView {
    Dawn(DawnView),
    Zenith(ZenithView),
    Twilight(TwilightView),
    Night(NightView),
    Eclipse(EclipseView),
}

impl SolarCasteView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        match self {
            SolarCasteView::Dawn(dawn) => dawn.has_caste_ability(ability),
            SolarCasteView::Zenith(zenith) => zenith.has_caste_ability(ability),
            SolarCasteView::Twilight(twilight) => twilight.has_caste_ability(ability),
            SolarCasteView::Night(night) => night.has_caste_ability(ability),
            SolarCasteView::Eclipse(eclipse) => eclipse.has_caste_ability(ability),
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        match self {
            SolarCasteView::Dawn(dawn) => dawn.supernal_ability(),
            SolarCasteView::Zenith(zenith) => zenith.supernal_ability(),
            SolarCasteView::Twilight(twilight) => twilight.supernal_ability(),
            SolarCasteView::Night(night) => night.supernal_ability(),
            SolarCasteView::Eclipse(eclipse) => eclipse.supernal_ability(),
        }
    }

    pub fn into_owned(self) -> SolarCasteMemo {
        match self {
            SolarCasteView::Dawn(view) => SolarCasteMemo::Dawn(view.into_owned()),
            SolarCasteView::Zenith(view) => SolarCasteMemo::Zenith(view.into_owned()),
            SolarCasteView::Twilight(view) => SolarCasteMemo::Twilight(view.into_owned()),
            SolarCasteView::Night(view) => SolarCasteMemo::Night(view.into_owned()),
            SolarCasteView::Eclipse(view) => SolarCasteMemo::Eclipse(view.into_owned()),
        }
    }
}
