use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DemenseName(String);

impl<T> From<T> for DemenseName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for DemenseName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}