use std::ops::Deref;

use super::named::NamedArtifactWeapon;

pub struct TwoHandedArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for TwoHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}