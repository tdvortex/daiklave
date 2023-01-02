use std::ops::Deref;

use super::named::NamedArtifactWeapon;

pub struct OneHandedArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for OneHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}