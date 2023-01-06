use std::ops::Deref;

use crate::weapons::weapon::artifact::named::NamedArtifactWeapon;

mod memo;

pub use memo::NaturalArtifactWeaponMemo;

/// An artifact weapon which is part of the user's body. (This is uncommon,
/// but occurs with weapons like the Blood Lash spell).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalArtifactWeapon<'source>(pub(crate) NamedArtifactWeapon<'source>);

impl<'source> Deref for NaturalArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> NaturalArtifactWeapon<'source> {
    pub(crate) fn as_memo(&'source self) -> NaturalArtifactWeaponMemo {
        NaturalArtifactWeaponMemo(self.0.as_memo())
    }
}
