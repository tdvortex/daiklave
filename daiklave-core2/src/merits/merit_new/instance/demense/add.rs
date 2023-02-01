use crate::hearthstones::hearthstone::GeomancyLevel;

use super::DemenseName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDemense {
    pub name: DemenseName,
    pub geomancy_level: GeomancyLevel,
}