use crate::{merits::merit::DemenseName, hearthstones::hearthstone::GeomancyLevel, CharacterMutation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDemense {
    pub name: DemenseName,
    pub geomancy_level: GeomancyLevel,
}

impl From<AddDemense> for CharacterMutation {
    fn from(add_demense: AddDemense) -> Self {
        Self::AddMerit(add_demense.into())
    }
}