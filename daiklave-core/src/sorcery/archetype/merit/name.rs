use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
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