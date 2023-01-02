use std::ops::Deref;

use super::named::NamedArtifactWeapon;

pub struct WornArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for WornArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}