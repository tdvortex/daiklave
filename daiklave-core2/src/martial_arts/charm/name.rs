use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// The name of a Martial Arts charm.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MartialArtsCharmName(String);

impl<T> From<T> for MartialArtsCharmName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for MartialArtsCharmName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
