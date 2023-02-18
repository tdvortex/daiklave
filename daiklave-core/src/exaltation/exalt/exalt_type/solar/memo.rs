use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityName, exaltation::exalt::LimitMemo, experience::ExperiencePool};

use super::{
    caste::SolarCasteMemo,
    charm::{SolarCharmDetails, SolarCharmName},
    Solar, SolarSorcererMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarMemo {
    pub caste: SolarCasteMemo,
    pub favored_abilities: [AbilityName; 5],
    pub sorcery: Option<SolarSorcererMemo>,
    pub limit: LimitMemo,
    pub solar_charms: Vec<(SolarCharmName, SolarCharmDetails)>,
    pub experience: ExperiencePool,
}

#[allow(clippy::from_over_into)]
impl<'source> Into<Solar<'source>> for &'source SolarMemo {
    fn into(self) -> Solar<'source> {
        Solar {
            caste: (&self.caste).into(),
            favored_abilities: self.favored_abilities,
            experience: self.experience,
            sorcery: self.sorcery.as_ref().map(|sorcery| (sorcery).into()),
            limit: (&self.limit).into(),
            solar_charms: self
                .solar_charms
                .iter()
                .map(|(charm_name, charm)| (charm_name.as_str(), charm))
                .collect(),
        }
    }
}

impl From<&Solar<'_>> for SolarMemo {
    fn from(value: &Solar<'_>) -> Self {
        Self {
            caste: (&value.caste).into(),
            favored_abilities: value.favored_abilities,
            sorcery: value.sorcery.as_ref().map(|view| view.into()),
            limit: (&value.limit).into(),
            solar_charms: value.solar_charms.iter().map(|(name, details)| ((*name).into(), (*details).to_owned())).collect(),
            experience: value.experience,
        }
    }
}