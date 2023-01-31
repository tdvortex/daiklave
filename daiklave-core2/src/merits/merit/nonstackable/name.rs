use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct NonStackableMeritName(String);

impl<T> From<T> for NonStackableMeritName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for NonStackableMeritName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}