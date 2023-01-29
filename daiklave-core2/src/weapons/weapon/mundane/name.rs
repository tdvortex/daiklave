use std::ops::Deref;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct MundaneWeaponName(String);

impl<T> From<T> for MundaneWeaponName where T: ToString {
    fn from(name: T) -> Self {
        Self(name.to_string())
    }
}

impl Deref for MundaneWeaponName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}