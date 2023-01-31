use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::hearthstones::hearthstone::GeomancyLevel;

use super::{AddDemense, RemoveDemense};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DemenseName(String);

impl DemenseName {
    pub fn with_level(self, geomancy_level: GeomancyLevel) -> AddDemense {
        AddDemense { name: self, geomancy_level }
    }

    pub fn remove(self) -> RemoveDemense {
        RemoveDemense::from(self)
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