use crate::{
    exalt_state::exalt::exalt_type::solar::{dawn::DawnView, zenith::ZenithView, twilight::TwilightView, night::NightView, eclipse::EclipseView}, abilities::AbilityName,
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
}