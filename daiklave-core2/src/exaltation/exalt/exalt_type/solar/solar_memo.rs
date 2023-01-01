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
    pub fn as_ref(&'source self) -> SolarView<'source> {
        SolarView {
            caste: self.caste.as_ref(),
            favored_abilities: self.favored_abilities,
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_ref()),
        }
    }
}