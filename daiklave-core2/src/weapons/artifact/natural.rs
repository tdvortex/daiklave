use std::ops::Deref;

use super::named::NamedArtifactWeapon;

pub struct NaturalArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for NaturalArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}