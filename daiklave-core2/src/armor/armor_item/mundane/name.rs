use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::armor::armor_item::builder::base::MundaneArmorBuilder;

use super::RemoveMundaneArmor;

/// The name of a piece of mundane armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneArmorName(String);

impl MundaneArmorName {
    /// Starts constructing an AddMundaneArmor mutation with this name.
    pub fn add(self) -> MundaneArmorBuilder {
        MundaneArmorBuilder::name(self)
    }

    /// Constructs a RemoveMundaneArmor mutation with this name.
    pub fn remove(self) -> RemoveMundaneArmor {
        self.into()
    }
}

impl<T> From<T> for MundaneArmorName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for MundaneArmorName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}