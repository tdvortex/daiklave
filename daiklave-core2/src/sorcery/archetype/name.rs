use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeName(String);

impl<T> From<T> for SorceryArchetypeName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for SorceryArchetypeName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}