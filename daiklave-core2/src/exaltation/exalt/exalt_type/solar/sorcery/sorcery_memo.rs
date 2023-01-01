use serde::{Deserialize, Serialize};

use crate::sorcery::circles::{
    celestial::sorcerer_memo::CelestialCircleSorcererMemo,
    solar::sorcerer_memo::SolarCircleSorcererMemo,
    terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo,
};

use super::SolarSorcererView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarSorcererMemo {
    Terrestrial(Box<TerrestrialCircleSorcererMemo>),
    Celestial(Box<CelestialCircleSorcererMemo>),
    Solar(Box<SolarCircleSorcererMemo>),
}

impl<'source> SolarSorcererMemo {
    pub fn as_ref(&'source self) -> SolarSorcererView<'source> {
        match self {
            SolarSorcererMemo::Terrestrial(box_memo) => {
                SolarSorcererView::Terrestrial(box_memo.as_ref().as_ref())
            }
            SolarSorcererMemo::Celestial(box_memo) => {
                SolarSorcererView::Celestial(box_memo.as_ref().as_ref())
            }
            SolarSorcererMemo::Solar(box_memo) => {
                SolarSorcererView::Solar(box_memo.as_ref().as_ref())
            }
        }
    }
}
