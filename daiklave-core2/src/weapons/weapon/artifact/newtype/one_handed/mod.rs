use std::ops::Deref;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

mod memo;
pub use memo::OneHandedArtifactWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OneHandedArtifactWeaponView<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for OneHandedArtifactWeaponView<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> OneHandedArtifactWeaponView<'source> {
    pub(crate) fn as_memo(&'source self) -> OneHandedArtifactWeapon {
        OneHandedArtifactWeapon(self.0.as_memo())
    }
}
