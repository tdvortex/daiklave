use std::ops::Deref;

use crate::id::UniqueId;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WeaponId(pub UniqueId);

impl Deref for WeaponId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}