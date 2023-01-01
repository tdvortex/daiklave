use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityName};

use super::{
    caste::SolarCasteMemo, SolarSorcererMemo, SolarView,
};


/// An owned copy of all Solar traits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarMemo {
    caste: SolarCasteMemo,
    favored_abilities: [AbilityName; 5],
    sorcery: Option<SolarSorcererMemo>,
}

impl<'source> SolarMemo {
    pub fn as_solar(&'source self) -> SolarView<'source> {
        todo!()
    }
}