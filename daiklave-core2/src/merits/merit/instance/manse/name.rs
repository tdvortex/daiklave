use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::RemoveManse;

/// The name of a Manse.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManseName(String);

impl ManseName {
    /// Creates a mutation to remove the Manse. Will also remove its associated
    /// Demense and Hearthstone.
    pub fn remove(self) -> RemoveManse {
        RemoveManse(self)
    }
}

impl<T> From<T> for ManseName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for ManseName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}