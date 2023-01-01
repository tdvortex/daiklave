use serde::{Serialize, Deserialize};

use crate::sorcery::{circles::{terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo, celestial::sorcerer_memo::CelestialCircleSorcererMemo, solar::sorcerer_memo::SolarCircleSorcererMemo}, 
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarSorcererMemo {
    Terrestrial(Box<TerrestrialCircleSorcererMemo>),
    Celestial(Box<CelestialCircleSorcererMemo>),
    Solar(Box<SolarCircleSorcererMemo>),
}