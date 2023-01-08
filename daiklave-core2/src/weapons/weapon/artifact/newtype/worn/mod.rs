mod memo;
pub use memo::WornArtifactWeapon;

use std::ops::Deref;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WornArtifactWeaponView<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for WornArtifactWeaponView<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> WornArtifactWeaponView<'source> {
    pub(crate) fn as_memo(&'source self) -> WornArtifactWeapon {
        WornArtifactWeapon(self.0.as_memo())
    }
}
