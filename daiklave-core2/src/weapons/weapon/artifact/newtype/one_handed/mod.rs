use std::ops::Deref;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

mod memo;
pub use memo::OneHandedArtifactWeaponMemo;

/// A one-handed artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedArtifactWeapon<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for OneHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> OneHandedArtifactWeapon<'source> {
    pub(crate) fn as_memo(&'source self) -> OneHandedArtifactWeaponMemo {
        OneHandedArtifactWeaponMemo(self.0.as_memo())
    }
}
