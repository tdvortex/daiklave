use serde::{Serialize, Deserialize};

use crate::{charms::charm::AddCharm, CharacterMutation};

use super::{SolarCharmDetails, SolarCharmName};

/// A Solar Charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddSolarCharm {
    pub(crate) name: SolarCharmName,
    pub(crate) details: SolarCharmDetails,
}

impl From<AddSolarCharm> for CharacterMutation {
    fn from(add_solar_charm: AddSolarCharm) -> Self {
        AddCharm::from(add_solar_charm).into()
    }
}
