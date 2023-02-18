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

impl From<&SolarSorcererView<'_>> for SolarSorcererMemo {
    fn from(value: &SolarSorcererView<'_>) -> Self {
        match value {
            SolarSorcererView::Terrestrial(terrestrial) => Self::Terrestrial(Box::new(terrestrial.into())),
            SolarSorcererView::Celestial(celestial) => Self::Celestial(Box::new(celestial.into())),
            SolarSorcererView::Solar(solar) => Self::Solar(Box::new(solar.into())),
        }
    }
}