mod memo;
pub use memo::WornArtifactWeaponMemo;

use std::ops::Deref;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

/// An artifact weapon that is worn when equipped, and does not use
/// any hands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WornArtifactWeapon<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for WornArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> WornArtifactWeapon<'source> {
    pub(crate) fn as_memo(&'source self) -> WornArtifactWeaponMemo {
        WornArtifactWeaponMemo(self.0.as_memo())
    }
}
