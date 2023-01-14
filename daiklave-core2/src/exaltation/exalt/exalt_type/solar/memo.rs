use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityName, exaltation::exalt::LimitMemo};

use super::{caste::SolarCasteMemo, Solar, SolarSorcererMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarMemo {
    pub caste: SolarCasteMemo,
    pub favored_abilities: [AbilityName; 5],
    pub sorcery: Option<SolarSorcererMemo>,
    pub limit: LimitMemo,
}

impl<'source> SolarMemo {
    pub(in crate::exaltation) fn as_ref(&'source self) -> Solar<'source> {
        Solar {
            caste: self.caste.as_ref(),
            favored_abilities: self.favored_abilities,
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_ref()),
            limit: self.limit.as_ref(),
        }
    }
}
