use crate::{CharacterMutation, charms::charm::AddCharm};

use super::{SolarCharm, SolarCharmName};

/// A Solar Charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSolarCharm {
    pub(crate) name: SolarCharmName,
    pub(crate) charm: SolarCharm,
}

impl From<AddSolarCharm> for CharacterMutation {
    fn from(add_solar_charm: AddSolarCharm) -> Self {
        AddCharm::from(add_solar_charm).into()
    }
}