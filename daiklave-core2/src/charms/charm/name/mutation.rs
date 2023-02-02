use serde::{Deserialize, Serialize};

use crate::{
    charms::charm::{evocation::EvocationName, spirit::SpiritCharmName},
    exaltation::exalt::exalt_type::solar::charm::SolarCharmName,
    martial_arts::charm::MartialArtsCharmName,
    sorcery::spell::SpellName,
};

use super::CharmName;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum CharmNameMutation {
    Spirit(SpiritCharmName),
    Evocation(EvocationName),
    MartialArts(MartialArtsCharmName),
    Solar(SolarCharmName),
    Spell(SpellName),
}

#[allow(clippy::from_over_into)]
impl<'source> Into<CharmName<'source>> for &'source CharmNameMutation {
    fn into(self) -> CharmName<'source> {
        match self {
            CharmNameMutation::Spirit(name) => CharmName::Spirit(name.as_str()),
            CharmNameMutation::Evocation(name) => CharmName::Evocation(name.as_str()),
            CharmNameMutation::MartialArts(name) => CharmName::MartialArts(name.as_str()),
            CharmNameMutation::Solar(name) => CharmName::Solar(name.as_str()),
            CharmNameMutation::Spell(name) => CharmName::Spell(name.as_str()),
        }
    }
}

impl From<CharmName<'_>> for CharmNameMutation {
    fn from(name: CharmName<'_>) -> Self {
        match name {
            CharmName::Spirit(name) => Self::Spirit(name.into()),
            CharmName::Evocation(name) => Self::Evocation(name.into()),
            CharmName::MartialArts(name) => Self::MartialArts(name.into()),
            CharmName::Solar(name) => Self::Solar(name.into()),
            CharmName::Spell(name) => Self::Spell(name.into()),
        }
    }
}
