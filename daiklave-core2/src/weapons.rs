use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::id::UniqueId;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct WeaponId(pub UniqueId);

impl Deref for WeaponId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}