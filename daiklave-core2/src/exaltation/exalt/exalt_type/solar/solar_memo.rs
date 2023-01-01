use serde::{Deserialize, Serialize};

use crate::abilities::AbilityName;

use super::{caste::SolarCasteMemo, SolarSorcererMemo, SolarView};

/// An owned copy of all Solar traits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarMemo {
    caste: SolarCasteMemo,
    favored_abilities: [AbilityName; 5],
    sorcery: Option<SolarSorcererMemo>,
}

impl<'source> SolarMemo {
    pub(in crate::exaltation::exalt::exalt_type::solar) fn new(
        caste: SolarCasteMemo,
        favored_abilities: [AbilityName; 5],
        sorcery: Option<SolarSorcererMemo>,
    ) -> Self {
        Self {
            caste,
            favored_abilities,
            sorcery,
        }
    }

    pub(in crate::exaltation) fn as_ref(&'source self) -> SolarView<'source> {
        SolarView::new(
            self.caste.as_ref(),
            self.favored_abilities,
            self.sorcery.as_ref().map(|sorcery| sorcery.as_ref()),
        )
    }
}
