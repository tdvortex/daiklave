use std::ops::Deref;

use serde::{Serialize, Deserialize};

/// The name of a Spirit Charm.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpiritCharmName(String);

impl<T> From<T> for SpiritCharmName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for SpiritCharmName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}