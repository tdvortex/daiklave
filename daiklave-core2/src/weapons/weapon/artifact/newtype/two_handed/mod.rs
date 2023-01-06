mod memo;
pub use memo::TwoHandedArtifactWeaponMemo;

use std::ops::Deref;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

/// A two-handed artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoHandedArtifactWeapon<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for TwoHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> TwoHandedArtifactWeapon<'source> {
    pub(crate) fn as_memo(&'source self) -> TwoHandedArtifactWeaponMemo {
        TwoHandedArtifactWeaponMemo(self.0.as_memo())
    }
}
