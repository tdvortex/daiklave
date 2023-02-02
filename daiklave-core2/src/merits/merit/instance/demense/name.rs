use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::{hearthstones::hearthstone::GeomancyLevel, merits::merit::RemoveMerit};

use super::{AddDemense};

/// The name of a demense.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DemenseName(String);

impl DemenseName {
    /// Constructs a new standalone Demense with the given name and level.
    pub fn with_level(self, geomancy_level: GeomancyLevel) -> AddDemense {
        AddDemense { name: self, geomancy_level }
    }

    /// Constructs a mutation to remove a demense with this name.
    pub fn remove(self) -> RemoveMerit {
        RemoveMerit::Demense(self)
    }
}

impl<T> From<T> for DemenseName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for DemenseName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}