use crate::{CharacterMutation, charms::charm::AddCharm};

use super::{SolarCharm, SolarCharmName};

/// A Solar Charm to be added to a character.
pub struct AddSolarCharm {
    name: SolarCharmName,
    charm: SolarCharm,
}

impl From<AddSolarCharm> for CharacterMutation {
    fn from(add_solar_charm: AddSolarCharm) -> Self {
        AddCharm::from(add_solar_charm).into()
    }
}