use std::ops::Deref;

use crate::id::Id;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WeaponId(pub Id);

impl Deref for WeaponId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}