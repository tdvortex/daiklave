use crate::{CharacterMutation, charms::charm::AddCharm};

use super::{SolarCharmName, SolarCharmDetails};

/// A Solar Charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSolarCharm {
    pub(crate) name: SolarCharmName,
    pub(crate) details: SolarCharmDetails,
}

impl From<AddSolarCharm> for CharacterMutation {
    fn from(add_solar_charm: AddSolarCharm) -> Self {
        AddCharm::from(add_solar_charm).into()
    }
}