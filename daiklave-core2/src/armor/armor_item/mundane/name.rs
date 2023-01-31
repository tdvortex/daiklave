use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneArmorName(String);

impl<T> From<T> for MundaneArmorName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for MundaneArmorName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}