use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HearthstoneName(String);

impl<T> From<T> for HearthstoneName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for HearthstoneName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}