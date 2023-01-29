use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SolarCharmName(String);

impl<T> From<T> for SolarCharmName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for SolarCharmName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}