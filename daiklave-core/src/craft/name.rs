use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CraftName(String);

impl<T> From<T> for CraftName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for CraftName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
