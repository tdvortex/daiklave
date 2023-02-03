use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::{builder::EvocationBuilderWithName, EvokableName};

/// The name of an Evocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvocationName(String);

impl EvocationName {
    /// Starts constructing the evocation by specifying what it is an
    /// evocation of.
    pub fn evocation_of(&self, evokable_name: EvokableName) -> EvocationBuilderWithName {
        evokable_name.with_evocation(self.clone())
    }
}

impl<T> From<T> for EvocationName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for EvocationName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
