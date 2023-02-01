use std::ops::Deref;

use serde::{Serialize, Deserialize};

/// The name of an Artifact piece of armor. This is the unique name of the
/// item, like "Brilliant Sentinel".
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ArtifactArmorName(String);

impl<T> From<T> for ArtifactArmorName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for ArtifactArmorName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}