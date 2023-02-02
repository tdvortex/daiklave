use std::ops::Deref;

use serde::{Serialize, Deserialize};

/// The name of a sorcery archetype merit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeMeritName(String);

impl<T> From<T> for SorceryArchetypeMeritName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for SorceryArchetypeMeritName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}