use serde::{Serialize, Deserialize};

use crate::{
    charms::charm::{spirit::SpiritCharmName, AddCharm},
    CharacterMutation,
};

use super::EclipseCharm;

/// An Eclipse Charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddEclipseCharm {
    pub(crate) name: SpiritCharmName,
    pub(crate) charm: EclipseCharm,
}

impl From<AddEclipseCharm> for CharacterMutation {
    fn from(add_eclipse: AddEclipseCharm) -> Self {
        AddCharm::from(add_eclipse).into()
    }
}
