mod memo;
pub use memo::TwoHandedArtifactWeapon;

use std::ops::Deref;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TwoHandedArtifactWeaponView<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for TwoHandedArtifactWeaponView<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> TwoHandedArtifactWeaponView<'source> {
    pub(crate) fn as_memo(&'source self) -> TwoHandedArtifactWeapon {
        TwoHandedArtifactWeapon(self.0.as_memo())
    }
}
