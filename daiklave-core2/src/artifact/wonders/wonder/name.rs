use std::ops::Deref;

use serde::{Serialize, Deserialize};

/// The name of a non-weapon, non-armor, non-warstrider artifact.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct WonderName(String);

impl<T> From<T> for WonderName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for WonderName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}