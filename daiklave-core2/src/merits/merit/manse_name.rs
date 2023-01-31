use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManseName(String);

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