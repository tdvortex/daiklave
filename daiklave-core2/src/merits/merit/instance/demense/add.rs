use crate::{hearthstones::hearthstone::GeomancyLevel, merits::merit::AddMerit, CharacterMutation};

use super::DemenseName;

/// A mutation to add a standalone Demense (no hearthstone or manse) to a
/// character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDemense {
    /// The name of the demense.
    pub name: DemenseName,
    /// Demense may be either Standard or Greater in potency.
    pub geomancy_level: GeomancyLevel,
}

impl From<AddDemense> for CharacterMutation {
    fn from(add_demense: AddDemense) -> Self {
        AddMerit::from(add_demense).into()
    }
}
