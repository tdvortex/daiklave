use serde::{Deserialize, Serialize};

use crate::sorcery::circles::{
    celestial::sorcerer_memo::CelestialCircleSorcererMemo,
    solar::sorcerer_memo::SolarCircleSorcererMemo,
    terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarSorcererMemo {
    Terrestrial(Box<TerrestrialCircleSorcererMemo>),
    Celestial(Box<CelestialCircleSorcererMemo>),
    Solar(Box<SolarCircleSorcererMemo>),
}
