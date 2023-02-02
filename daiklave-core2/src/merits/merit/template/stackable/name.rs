use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// The name of a stackable merit that may be purchased by a character.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StackableMeritTemplateName(String);

impl<T> From<T> for StackableMeritTemplateName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for StackableMeritTemplateName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
