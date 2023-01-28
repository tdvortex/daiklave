use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityName, exaltation::exalt::LimitMemo, experience::ExperiencePool};

use super::{
    caste::SolarCasteMemo,
    charm::{SolarCharm},
    Solar, SolarSorcererMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarMemo {
    pub caste: SolarCasteMemo,
    pub favored_abilities: [AbilityName; 5],
    pub sorcery: Option<SolarSorcererMemo>,
    pub limit: LimitMemo,
    pub solar_charms: Vec<(String, SolarCharm)>,
    pub experience: ExperiencePool,
}

impl<'source> SolarMemo {
    pub(in crate::exaltation) fn as_ref(&'source self) -> Solar<'source> {
        Solar {
            caste: self.caste.as_ref(),
            favored_abilities: self.favored_abilities,
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_ref()),
            limit: self.limit.as_ref(),
            solar_charms: self
                .solar_charms
                .iter()
                .map(|(charm_id, charm)| (charm_id.as_str(), charm))
                .collect(),
            experience: self.experience,
        }
    }
}
