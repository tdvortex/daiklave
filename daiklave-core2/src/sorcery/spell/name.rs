use std::ops::Deref;

use serde::{Serialize, Deserialize};

/// The name of a spell.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellName(String);

impl<T> From<T> for SpellName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for SpellName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}